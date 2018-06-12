use std::mem::swap;

use manager::ManagedWindow;
use settings::Settings;

use xrb::XClient;
use xrb::models::*;

#[derive(Debug)]
pub struct Tiled {
    pub children: Vec<TiledChild>,
    direction: TiledDirection,
    offset: i16, // Pixels
    dirty: bool // If this should be redrawn
}
impl Tiled {
    pub fn new_0(direction: TiledDirection) -> Tiled {
        Tiled {
            children: Vec::with_capacity(2),
            direction,
            offset: 0,
            dirty: true
        }
    }

    pub fn new_1(window: ManagedWindow, direction: TiledDirection) -> Tiled {
        let mut vec = Vec::with_capacity(2);
        vec.push(TiledChild::Window(window));
        Tiled {
            children: vec,
            direction,
            offset: 0,
            dirty: true
        }
    }

    pub fn new_2(window1: ManagedWindow, window2: ManagedWindow, direction: TiledDirection) -> Tiled {
        let mut vec = Vec::with_capacity(2);
        vec.push(TiledChild::Window(window1));
        vec.push(TiledChild::Window(window2));
        Tiled {
            children: vec,
            direction,
            offset: 0,
            dirty: true
        }
    }

    /** Marks this grid as dirty. */
    pub fn mark_dirty(&mut self) {
        self.dirty = true;
    }

    /** Returns true if this Tiled is dirty. */
    pub fn is_dirty(&self) -> bool {
        return self.dirty;
    }

    /** Swaps the position of window_0 and window_1, or does nothing if window_1 is empty. */
    pub fn swap(&mut self) {
        if self.children.len() == 2 {
            self.children.swap(0, 1);
        }

        self.mark_dirty();
    }

    /**
     * Adds a window at the currently focused window.
     * This Tiled may be dirty after this request, and as much may need to be tile()'d.
     */
    pub fn add(&mut self, mut window: ManagedWindow, direction: TiledDirection) {
        // If there is no children, just add it (will only happen for the windows directly under a workspace)
        if self.children.len() == 0 {
            self.children.push(TiledChild::Window(window));
            self.direction = direction;

            self.mark_dirty();
            return;
        }

        // Get directions to the currently focused window
        let mut directions = Vec::with_capacity(8); // Default to 8 layers. Might be more, but that would be a LOT of windows in one workspace.
        if !self.locate_focused(&mut directions) {
            eprintln!("WARNING: Failed to locate focused window, but windows exist. Adding to first.");
            if self.add_emergency(window, direction).is_err() {
                eprintln!("ERROR: Window not created. What the fuck happened? Did you throw your computer in the ocean before running this? Or did I fuck something up that bad? In any case, please submit an issue. And maybe call an exorcist.");
            }

            return;
        }

        // Add to the currently focused window
        self.add_from_directions(window, direction, &directions, 0);
        self.mark_dirty();
    }

    /**
     * Adds a window from the given directions
     */
    fn add_from_directions(&mut self, mut window: ManagedWindow, direction: TiledDirection, directions: &Vec<usize>, pos: usize) {
        if pos == directions.len() - 1 {
            let mut focused = self.children.remove(directions[pos]).unwrap_window(); // Assumes directions are correct
            focused.focused = false;
            window.focused = true;
            let mut new = Tiled::new_2(focused, window, direction);
            new.mark_dirty();
            self.children.insert(directions[pos], TiledChild::Tiled(new));
        } else {
            match self.children.get_mut(directions[pos]) {
                Some(tiled) => tiled.get_tiled().add_from_directions(window, direction, directions, pos + 1), // Assumes directions are correct
                None => unreachable!()
            };
        }

        self.mark_dirty();
    }

    /**
     * Attempts to add when there is no focused window, but a window exists.
     */
    fn add_emergency(&mut self, mut window: ManagedWindow, direction: TiledDirection) -> Result<(), ManagedWindow> {
        if self.children.len() == 2 {
            for index in 0..self.children.len() {
                let child = self.children.remove(index);

                if child.is_window() {
                    let mut old = child.unwrap_window();
                    old.focused = false;
                    window.focused = true;
                    let mut new = Tiled::new_2(old, window, direction);
                    new.mark_dirty();
                    self.children.push(TiledChild::Tiled(new));

                    self.mark_dirty();
                    return Ok(());
                } else { // child.is_tiled()
                    let mut tiled = child.unwrap_tiled();
                    let res = tiled.add_emergency(window, direction);

                    if res.is_ok() {
                        self.mark_dirty();
                        return Ok(());
                    } else { // res.is_err()
                        window = res.unwrap_err();
                        self.children.insert(index, TiledChild::Tiled(tiled));
                    }
                }
            }
        } else {
            self.children.push(TiledChild::Window(window));
            self.direction = direction;

            self.mark_dirty();
            return Ok(());
        }

        return Err(window);
    }

    /**
     * Locates the currently focused window, returning indexes to get to it.
     */
    pub fn locate_focused(&self, directions: &mut Vec<usize>) -> bool {
        for (i, child) in self.children.iter().enumerate() {
            match child {
                TiledChild::Window(window) => {
                    if window.focused {
                        directions.push(i);
                        return true;
                    }
                },
                TiledChild::Tiled(tiled) => {
                    directions.push(i);

                    if tiled.locate_focused(directions) {
                        return true;
                    } else {
                        directions.pop();
                    }
                }
            }
        }

        return false;
    }

    /** Removes the window given its ID. Returns true if something was removed. */
    pub fn remove(&mut self, wid: u32) -> bool {
        // TODO: Focus new window when this one gets removed
        let mut index = usize::max_value();
        let mut tiled_children = 0;

        for (i, child) in self.children.iter_mut().enumerate() {
            match child {
                TiledChild::Window(window) => {
                    if window.window.wid == wid {
                        index = i;
                        break;
                    }
                },
                TiledChild::Tiled(tiled) => {
                    if tiled.remove(wid) {
                        index = i;
                        tiled_children = tiled.children.len(); // Will never be 0
                        break;
                    }
                }
            };
        }

        if tiled_children == 1 {
            let mut tiled = self.children.remove(index).unwrap_tiled();
            self.children.insert(index, tiled.children.remove(0));
            self.set_first_focused();
            self.mark_dirty();
            return true;
        } else if tiled_children == 2 {
            self.mark_dirty();
            return true; // Will be 0 if nothing was removed
        }

        if index != usize::max_value() {
            self.children.remove(index);

            if self.children.len() == 1 {
                if self.children[0].is_tiled() {
                    let mut child = self.children.remove(0).unwrap_tiled();
                    for _ in 0..child.children.len() {
                        self.children.push(child.children.remove(0));
                    }
                }
            }

            self.mark_dirty();
            return true;
        }

        return false;
    }

    /** Returns the given window given its ID */
    pub fn get_window(&self, wid: u32) -> Option<&ManagedWindow> {
        for win in self.children.iter() {
            match win {
                TiledChild::Window(wrapped) => {
                    if wrapped.window.wid == wid || wrapped.window.wid == wid {
                        return Some(wrapped);
                    }
                },
                TiledChild::Tiled(tiled) => {
                    let res = tiled.get_window(wid);
                    if res.is_some() {
                        return res;
                    }
                }
            }
        };
        return None;
    }

    /** Returns the given window given its ID */
    pub fn get_window_mut(&mut self, wid: u32) -> Option<&mut ManagedWindow> {
        for win in self.children.iter_mut() {
            match win {
                TiledChild::Window(wrapped) => {
                    if wrapped.window.wid == wid || wrapped.wrapper.wid == wid {
                        return Some(wrapped);
                    }
                },
                TiledChild::Tiled(tiled) => {
                    let res = tiled.get_window_mut(wid);
                    if res.is_some() {
                        return res;
                    }
                }
            };
        }
        return None;
    }

    /** Makes the given window focused. Returns true if something was modified and is now dirty (parents should also be marked dirty). */
    pub fn set_focused(&mut self, wid: u32) -> bool {
        let mut found = false;

        for win in self.children.iter_mut() {
            match win {
                TiledChild::Window(wrapped) => {
                    if wrapped.window.wid == wid || wrapped.wrapper.wid == wid {
                        wrapped.focused = true;
                        found = true;
                    } else {
                        wrapped.focused = false;
                    }
                },
                TiledChild::Tiled(tiled) => {
                    if tiled.set_focused(wid) {
                        found = true;
                    }
                }
            }
        }

        self.mark_dirty();
        return found;
    }

    /** Makes the first window found focused. */
    pub fn set_first_focused(&mut self) {
        {
            let child = self.children.get_mut(0).unwrap(); // Must always have at least one

            match child {
                TiledChild::Window(win) => win.focused = true,
                TiledChild::Tiled(tiled) => tiled.set_first_focused()
            };
        }

        self.mark_dirty();
    }

    /** Returns the currently focused Window, or None */
    pub fn get_focused(&self) -> Option<&ManagedWindow> {
        for win in self.children.iter() {
            match win {
                TiledChild::Window(wrapped) => {
                    if wrapped.focused {
                        return Some(&wrapped);
                    }
                },
                TiledChild::Tiled(tiled) => {
                    let res = tiled.get_focused();
                    if res.is_some() {
                        return res;
                    }
                }
            }
        }

        return None;
    }

    /** Returns the currently focused Window, or None */
    pub fn get_focused_mut(&mut self) -> Option<&mut ManagedWindow> {
        for win in self.children.iter_mut() {
            match win {
                TiledChild::Window(wrapped) => {
                    if wrapped.focused {
                        return Some(wrapped);
                    }
                },
                TiledChild::Tiled(tiled) => {
                    let res = tiled.get_focused_mut();
                    if res.is_some() {
                        return res;
                    }
                }
            }
        }

        return None;
    }

    /** Positions the related windows, recursively tiling its children. */
    pub fn tile(&mut self, client: &mut XClient, gc: &mut GraphicsContext, workspace_wid: u32, workspace_depth: u8, settings: &Settings, x: i16, y: i16, width: u16, height: u16, force: bool) {
        // Ensure we should actually do this
        if !self.dirty && !force {
            return;
        }
        if self.children.len() == 0 {
            return;
        };

        // Tile
        if self.children.len() == 1 { // Fill
            match &mut self.children[0] {
                TiledChild::Window(wrapped) => {
                    wrapped.wrapper.configure_multiple(
                        client,
                        vec![
                            WindowConfigureValue::X(x),
                            WindowConfigureValue::Y(y),
                            WindowConfigureValue::Width(width),
                            WindowConfigureValue::Height(height)
                        ]
                    );
                    wrapped.window.configure_multiple(
                        client,
                        vec![
                            WindowConfigureValue::X(0),
                            WindowConfigureValue::Y(20),
                            WindowConfigureValue::Width(width - 1),
                            WindowConfigureValue::Height(height - 20 - 1)
                        ]
                    );

                    // Tell the X Server to update this window
                    client.send_event(&ServerEvent::Expose {
                        window: wrapped.wrapper.wid,
                        x: 0,
                        y: 0,
                        width: wrapped.wrapper.width,
                        height: wrapped.wrapper.height,
                        count: 0
                    }, false, wrapped.wrapper.wid, &vec![]);
                    //wrapped.paint(client, gc, workspace_wid, workspace_depth, &settings)
                },
                TiledChild::Tiled(child) => {
                    child.tile(client, gc, workspace_wid, workspace_depth, &settings, x, y, width, height, force);
                }
            }
        } else {
            // Get the children and their positions
            let (
                first_x, first_y, first_width, first_height,
                second_x, second_y, second_width, second_height
            ) = match self.direction {
                TiledDirection::Vertical => {(
                        x, y, width, (height as i16 / 2 + self.offset) as u16,
                        x, y + height as i16 / 2 + self.offset, width, (height as i16 / 2 - self.offset) as u16
                    )
                },
                TiledDirection::Horizontal => {
                    (
                        x, y, (width as i16 / 2 + self.offset) as u16, height,
                        x + width as i16 / 2 + self.offset, y, (width as i16 / 2 - self.offset) as u16, height
                    )
                }
            };

            // Apply changes
            match &mut self.children[0] {
                TiledChild::Window(wrapped) => {
                    wrapped.wrapper.configure_multiple(
                        client,
                        vec![
                            WindowConfigureValue::X(first_x),
                            WindowConfigureValue::Y(first_y),
                            WindowConfigureValue::Width(first_width),
                            WindowConfigureValue::Height(first_height)
                        ]
                    );
                    wrapped.window.configure_multiple(
                        client,
                        vec![
                            WindowConfigureValue::X(0),
                            WindowConfigureValue::Y(20),
                            WindowConfigureValue::Width(first_width - 1),
                            WindowConfigureValue::Height(first_height - 20 - 1)
                        ]
                    );

                    // Tell the X Server to update this window
                    client.send_event(&ServerEvent::Expose {
                        window: wrapped.wrapper.wid,
                        x: 0,
                        y: 0,
                        width: wrapped.wrapper.width,
                        height: wrapped.wrapper.height,
                        count: 0
                    }, false, wrapped.wrapper.wid, &vec![]);
                    //wrapped.paint(client, gc, workspace_wid, workspace_depth, &settings)
                },
                TiledChild::Tiled(child) => {
                    child.tile(client, gc, workspace_wid, workspace_depth, &settings, first_x, first_y, first_width, first_height, force);
                }
            };
            match &mut self.children[1] {
                TiledChild::Window(wrapped) => {
                    wrapped.wrapper.configure_multiple(
                        client,
                        vec![
                            WindowConfigureValue::X(second_x),
                            WindowConfigureValue::Y(second_y),
                            WindowConfigureValue::Width(second_width),
                            WindowConfigureValue::Height(second_height)
                        ]
                    );
                    wrapped.window.configure_multiple( // TODO: Take off border
                        client,
                        vec![
                            WindowConfigureValue::X(0),
                            WindowConfigureValue::Y(20),
                            WindowConfigureValue::Width(second_width - 1),
                            WindowConfigureValue::Height(second_height - 20 - 1)
                        ]
                    );

                    // Tell the X Server to update this window
                    client.send_event(&ServerEvent::Expose {
                        window: wrapped.wrapper.wid,
                        x: 0,
                        y: 0,
                        width: wrapped.wrapper.width,
                        height: wrapped.wrapper.height,
                        count: 0
                    }, false, wrapped.wrapper.wid, &vec![]);
                    //wrapped.paint(client, gc, workspace_wid, workspace_depth, &settings)
                },
                TiledChild::Tiled(child) => {
                    child.tile(client, gc, workspace_wid, workspace_depth, &settings, second_x, second_y, second_width, second_height, force);
                }
            };
        };

        // Mark as clean
        self.dirty = false;
    }
}

#[derive(Debug)]
pub enum TiledChild {
    Window(ManagedWindow),
    Tiled(Tiled)
}
impl TiledChild {
    fn is_window(&self) -> bool {
        match *self {
            TiledChild::Window(_) => true,
            _ => false
        }
    }

    fn is_tiled(&self) -> bool {
        match *self {
            TiledChild::Tiled(_) => true,
            _ => false
        }
    }

    fn unwrap_window(self) -> ManagedWindow {
        match self {
            TiledChild::Window(window) => window,
            _ => panic!("TiledChild.unwrap_window called on non-window")
        }
    }

    fn unwrap_tiled(self) -> Tiled {
        match self {
            TiledChild::Tiled(tiled) => tiled,
            _ => panic!("TiledChild.unwrap_tiled called on non-tiled")
        }
    }

    fn get_window(&mut self) -> &mut ManagedWindow {
        match self {
            TiledChild::Window(window) => window,
            _ => panic!("TiledChild.unwrap_window called on non-window")
        }
    }

    fn get_tiled(&mut self) -> &mut Tiled {
        match self {
            TiledChild::Tiled(tiled) => tiled,
            _ => panic!("TiledChild.unwrap_tiled called on non-tiled")
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum TiledDirection {
    Vertical,
    Horizontal
}
