const addon = require('./index.node');

class WithChannel {
    constructor(name) {
        this.class = addon.with_channel_new(name);
    }
    name() {
        return addon.with_channel_name.apply(this.class);
    }
}

class WithoutChannel {
    constructor(name) {
        this.class = addon.without_channel_new(name);
    }
    name() {
        return addon.without_channel_name.apply(this.class);
    }
}

// without event queue works fine
// setTimeout(() => {
//     console.log('Finished:', new WithoutChannel('Test').name())
//     // }, 100); // small timeout, process will exit
// }, 10000); // timeout around 10 seconds or higher, process will also exit

// with event queue it hangs after > ~10 seconds timeout
setTimeout(() => {
    console.log('Finished:', new WithChannel('Test').name())
    // }, 100); // small timeout, process will exit
}, 10000); // timeout around 10 seconds or higher, process will never exit
