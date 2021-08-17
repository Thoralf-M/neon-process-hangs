const addon = require('./index.node');

class WithEventQueue {
    constructor(name) {
        this.class = addon.with_event_queue_new(name);
    }
    name() {
        return addon.with_event_queue_name.apply(this.class);
    }
}

class WithoutEventQueue {
    constructor(name) {
        this.class = addon.without_event_queue_new(name);
    }
    name() {
        return addon.without_event_queue_name.apply(this.class);
    }
}

// without event queue works fine
// setTimeout(() => {
//     console.log('Finished:', new WithoutEventQueue('Test').name())
//     // }, 100); // small timeout, process will exit
// }, 10000); // timeout around 10 seconds or higher, process will also exit

// with event queue it hangs after > ~10 seconds timeout
setTimeout(() => {
    console.log('Finished:', new WithEventQueue('Test').name())
    // }, 100); // small timeout, process will exit
}, 10000); // timeout around 10 seconds or higher, process will never exit
