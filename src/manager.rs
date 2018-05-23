extern crate xrb;

use xrb::XClient;
use xrb::models::*;

use settings::Settings;

pub struct WindowManager {
    client: XClient,
    workspaces: Vec<Workspace>,
    current_workspace: usize,
    gc: GraphicsContext,
    settings: Settings,
}
impl WindowManager {

    /**
     * Creates a new WindowManager.
     * client should already be authenticated. This will set the bitmask.
     */
    pub fn new(mut client: XClient) -> WindowManager {
        // Create the graphics context
        let root = client.info.screens[0].root;
        let black = client.info.screens[0].black_pixel;
        let gc = GraphicsContext::create(
            &mut client,
            root,
            vec![
                GraphicsContextValue::Background(black),
                GraphicsContextValue::Foreground(black)
            ]
        );

        // Subscribe to all events
        let root_id = client.info.screens[0].root;
        let mut root = match Window::get_sync(&mut client, root_id) {
            Ok(win) => win,
            Err(err) => panic!("Failed to subscribe to root pane: {:?}", err)
        };
        root.set_multiple(&mut client, vec![
            WindowValue::EventMask(Event::KeyPress.val() | Event::SubstructureRedirect.val() | Event::FocusChange.val() | Event::PropertyChange.val()) // TODO: Should this be done per-window?
        ]);

        // Create the manager
        let workspaces = Vec::with_capacity(client.info.screens.len());
        let mut manager = WindowManager {
            client,
            workspaces: workspaces,
            current_workspace: 0,
            gc,
            settings: Settings::default()
        };

        // Create initial workspaces
        for i in 0..manager.client.info.screens.len() {
            manager.create_workspace(i as u32 + 1, i);
        }

        manager
    }

    /**
     * Presumably used during ServerEvent::MapRequest.
     * Reparents the given window and maps it.
     */
    pub fn add_window(&mut self, mut window: Window, parent: Window) {
        {
            let workspace = &mut self.workspaces[self.current_workspace];

            // Create the background (no contents; it will be filled during tile())
            let bg = Pixmap { // TODO: Pixmap::create
                depth: workspace.window.depth,
                pid: self.client.new_resource_id(),
                drawable: workspace.window.wid,
                width: window.width,
                height: window.height + 20
            };
            self.client.create_pixmap(&bg);

            // Create wrapper
            let wrapper = Window::create(
                &mut self.client,
                //parent.wid,
                workspace.window.wid,
                window.depth,
                window.x,
                window.y,
                window.width,
                window.height + 20,
                0,
                window.class,
                0, // CopyFromParent
                vec![
                    WindowValue::BackgroundPixmap(bg.pid),
                    WindowValue::Colormap(0x0),
                    WindowValue::EventMask(
                        Event::Button1Motion.val()
                    )
                ]
            );

            // Put window inside wrapper and map
            window.reparent(&mut self.client, wrapper.wid, 0, 20);
            window.map(&mut self.client);
            wrapper.map(&mut self.client);

            // Get the window's name
            let name = match window.get_string_sync(&mut self.client, DefaultAtom::WmName.val(), 200) {
                Some(name) => {
                    name
                },
                None => String::from("")
            };
            println!("Window has name: {}", name);

            // Add to list of windows
            workspace.windows.push(ManagedWindow {
                window,
                wrapper,
                parent,
                name,
                bg
            });
        }

        // Re-tile
        self.tile();
    }

    /**
     * Creates an empty workspace on the given screen.
     * Returns true if the operation succeeded and false if not.
     */
    pub fn create_workspace(&mut self, id: u32, screen: usize) -> bool {
        for workspace in self.workspaces.iter() {
            if workspace.id == id {
                return false;
            }
        }

        let root = self.client.info.screens[screen].root;
        let depth = self.client.info.screens[screen].root_depth;
        let width = self.client.info.screens[screen].width_in_pixels;
        let height = self.client.info.screens[screen].height_in_pixels;
        let visual = self.client.info.screens[screen].root_visual;
        self.workspaces.push(Workspace {
            id,
            window: Window::create(
                &mut self.client,
                root,
                depth,
                0,
                0,
                width,
                height,
                0,
                WindowInputType::CopyFromParent,
                visual,
                vec![]
            ),
            windows: vec![]
        });

        let new_index = self.workspaces.len() - 1;
        self.set_workspace(new_index);
        return true;
    }

    /**
     * Switches to the given workspace (unmaps old workspace, maps others)
     * Returns true if the operation succeeded, and false if not.
     */
    pub fn set_workspace(&mut self, workspace: usize) -> bool {
        // Ensure the given workspace is valid
        if workspace >= self.workspaces.len() {
            return false;
        }

        // Swap
        self.workspaces[self.current_workspace].window.unmap(&mut self.client);
        self.current_workspace = workspace;
        self.workspaces[self.current_workspace].window.map(&mut self.client);

        // Focus mouse
        // TODO: Focus mouse

        // Return true
        return true;
    }

    /**
     * Tiles the registered windows.
     */
    pub fn tile(&mut self) {
        // TODO: Actually tile well
        for workspace in self.workspaces.iter_mut() {
            // TODO: Actual window positioning
            let count = workspace.windows.len();
            let max_width = workspace.window.width;
            let max_height = workspace.window.height;

            for (i, wrapped) in workspace.windows.iter_mut().enumerate() {
                let new_x = 0;
                let new_y = i as i16 * max_height as i16 / count as i16;
                let new_width = max_width;
                let new_height = max_height / count as u16;

                // New pixmap
                self.client.free_pixmap(wrapped.bg.pid);
                let bg = Pixmap { // TODO: Pixmap::create
                    depth: workspace.window.depth,
                    pid: self.client.new_resource_id(),
                    drawable: workspace.window.wid,
                    width: new_width,
                    height: new_height
                };
                self.client.create_pixmap(&bg);

                // Title
                self.gc.set_fg(&mut self.client, &self.settings.win_title_bg);
                bg.fill_rect(&mut self.client, self.gc.gcid, Rectangle {
                    x: self.settings.win_title_border_width as i16,
                    y: self.settings.win_title_border_width as i16,
                    width: new_width - self.settings.win_title_border_width * 2,
                    height: 20 - self.settings.win_title_border_width * 2
                });

                self.gc.set_fg(&mut self.client, &self.settings.win_title_fg);
                bg.img_text8(&mut self.client, self.gc.gcid, &wrapped.name, 0, 10 );

                let border_width = self.settings.win_title_border_width;
                if border_width > 0 {
                    self.gc.set_fg(&mut self.client, &self.settings.win_title_border_color);
                    let mut rects = Vec::with_capacity(border_width as usize);
                    
                    for i in 0..border_width {
                        rects.push(Rectangle {
                            x: i as i16,
                            y: i as i16,
                            width: new_width - i * 2,
                            height: 20 - i * 2
                        });
                    }

                    bg.draw_rects(&mut self.client, self.gc.gcid, &rects);
                }

                // Background
                self.gc.set_fg(&mut self.client, &self.settings.win_bg);
                bg.fill_rect(&mut self.client, self.gc.gcid, Rectangle {
                    x: 1,
                    y: 20 + 1,
                    width: new_width - 2,
                    height: new_height - 20
                });

                let border_width = self.settings.win_border_width;
                if border_width > 0 {
                    self.gc.set_fg(&mut self.client, &self.settings.win_border_color);
                    let mut rects = Vec::with_capacity(border_width as usize);
                    
                    for i in 0..border_width {
                        rects.push(Rectangle {
                            x: i as i16,
                            y: i as i16,
                            width: new_width - i * 2,
                            height: new_height - i * 2
                        });
                    }

                    bg.draw_rects(&mut self.client, self.gc.gcid, &rects);
                }

                // Apply new background
                wrapped.wrapper.set(&mut self.client, WindowValue::BackgroundPixmap(bg.pid));
                wrapped.bg = bg;

                // Resize
                wrapped.wrapper.configure_multiple(
                    &mut self.client,
                    vec![
                        WindowConfigureValue::X(new_x),
                        WindowConfigureValue::Y(new_y),
                        WindowConfigureValue::Width(new_width),
                        WindowConfigureValue::Height(new_height)
                    ]
                );
                wrapped.window.configure_multiple(
                    &mut self.client,
                    vec![
                        WindowConfigureValue::X(0),
                        WindowConfigureValue::Y(20),
                        WindowConfigureValue::Width(new_width),
                        WindowConfigureValue::Height(new_height - 20)
                    ]
                );
            }
        }
    }

    /**
     * Starts listening to the event loop. This function loops forever and will never end.
     */
    pub fn run(&mut self) {
        loop {
            match self.client.wait_for_message() {
                ServerResponse::Error(error, sequence_number) => {
                    println!("Got error {}: {:?}", sequence_number, error);
                },
                ServerResponse::Reply(reply, sequence_number) => {
                    println!("Got reply {}: {:?}", sequence_number, reply);
                },
                ServerResponse::Event(event, sequence_number) => {
                    println!("Got event {}: {:?}", sequence_number, event);
                    match event {
                        ServerEvent::MapRequest { parent, window } => {
                            // Get the windows
                            let parent = Window::get_sync(&mut self.client, parent).unwrap();
                            let mut window = Window::get_sync(&mut self.client, window).unwrap();
                            
                            // Wrap
                            self.add_window(window, parent);
                        },
                        ServerEvent::KeyPress { key_code, time, root, event, child, root_x, root_y, event_x, event_y, state, same_screen } => {
                            // TODO: Actual keybindings
                            if key_code >= 10 && key_code <= 18 { // No 0 for now
                                let id = key_code as u32 - 9;
                                let mut index = self.workspaces.len();

                                for (i, workspace) in self.workspaces.iter().enumerate() {
                                    if workspace.id == id {
                                        index = i;
                                        break;
                                    }
                                }

                                if index == self.workspaces.len() {
                                    self.create_workspace(id, 0); // TODO: Use current screen
                                } else {
                                    self.set_workspace(index);
                                }
                            }
                        },
                        _ => () // TODO: More events
                    };
                }
            }
        }
    }

}

pub struct Workspace {
    id: u32,
    window: Window,
    windows: Vec<ManagedWindow>
}

pub struct ManagedWindow {
    window: Window,
    wrapper: Window,
    parent: Window,
    name: String,
    bg: Pixmap
}
