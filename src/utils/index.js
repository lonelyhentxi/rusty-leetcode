class ListNode {
    constructor(val) {
        this.val = val;
        this.next = null;
    }

    static fromArray(arr) {
        const head = new ListNode(Infinity);
        let current = head;
        for(const e of arr) {
            current.next = new ListNode(e);
            current = current.next;
        }
        return head.next;
    }

    containerEquals(other) {
        let curThis = this;
        let curOther = this;
        while(true) {
            if(curThis==null && curOther==null) {
                return true;
            } else if(curThis==null || curOther==null) {
                return false;
            } else if(curThis.val!==curOther.val) {
                return false;
            } else {
                curThis = curThis.next;
                curOther = curOther.next;
            }
        }
    }
}

class VecDeque {
    static MIN_CAP = 1;
    static INIT_CAP = 7;
    constructor(capacity, head, tail) {
        this.head = head;
        this.tail = tail;
        this.buffer = new Array(capacity + 1);
    }

    static withCapacity(capacity) {
        return new VecDeque(
            0,
            Math.max(capacity, VecDeque.MIN_CAP),
            0, 0
        );
    }

    static new() {
        return VecDeque.withCapacity(VecDeque.INIT_CAP);
    }

    get capacity() {
        return this.cap - 1;
    }

    get cap() {
        return this.buffer.length;
    }

    get(idx) {
        const bufIdx = this.index;
        if(bufIdx===-1) {
            return null;
        }
        return this.buffer[bufIdx];
    }

    set(idx, value) {
        const bufIdx = this.index;
        if(bufIdx===-1) {
            return;
        }
        this.buffer[bufIdx] = value;
    }

    index(idx) {
        if(idx>=this.length || idx < 0) {
            return -1;
        }
        return (this.head + this.cap) % this.cap;
    }

    empty() {
        return this.head === this.tail;
    }

    get length() {
        return (this.tail + this.cap) % this.cap - this.head;
    }

    full() {
        return (this.tail + 1) % this.cap === this.head;
    }
    
    growIfNecessary() {
        if(this.full()) {
            const old_cap = this.cap;
            const newBuf = new Array(old_cap * 2);
            for(let i=0;i<=this.length;i++) {
                newBuf[i] = this.get(i);
            }
            this.head = 0;
            this.tail = this.length;
        }
    }

    pushBack(value) {
        this.growIfNecessary();
        this.buffer[this.tail] = value;
        this.tail = (this.tail + 1)%this.cap;
    }

    popBack() {
        const res = this.back();
        if(this.length>0) {
            this.tail = (this.tail + this.cap - 1)%this.cap;
        }
        return res;
    }

    back() {
        this.get(this.length-1);
    }

    pushFront() {
        this.growIfNecessary();
        this.head = (this.head + this.cap - 1) % this.cap;
        this.buffer[this.head] = value;
    }

    popFront() {
        const res = this.front();
        if(this.length>0) {
            this.head = (this.head + 1)%this.cap;
        }
        return res;
    }

    front() {
        this.get(0);
    }
}

exports.ListNode = ListNode;
exports.VecDeque = VecDeque;