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

exports.ListNode = ListNode;