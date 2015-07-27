(function() {var implementors = {};
implementors['log'] = [];implementors['winapi'] = [];implementors['glium'] = [];implementors['lux'] = [];

            if (window.register_implementors) {
                window.register_implementors(implementors);
            } else {
                window.pending_implementors = implementors;
            }
        
})()
