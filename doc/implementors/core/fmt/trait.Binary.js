(function() {var implementors = {};
implementors['wayland_client'] = [];implementors['wayland_window'] = [];implementors['wayland_kbd'] = [];implementors['glutin'] = [];

            if (window.register_implementors) {
                window.register_implementors(implementors);
            } else {
                window.pending_implementors = implementors;
            }
        
})()
