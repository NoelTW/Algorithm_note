class Node {
    constructor(value) {
        this.value = value;
        this.next = null;
    }
}

class LinkedList {
    /**
     * This class represents a linked list data structure.
     */
    constructor() {
        this.head = null;
        this.tail = null;
        this.length = 0;
    }

    _traverse(index) {
        let currNode = this.head;
        let i = 0;
        while (i < index) {
            currNode = currNode.next;
            i++;
        }
        return currNode;
    }
    toArray() {
        const array = new Array(this.length);
        let currNode = this.head;
        let i = 0;
        while (currNode) {
            array[i] = currNode.value;
            currNode = currNode.next;
            i++;
        }
        return array;
    }
    append(value) {
        const newNode = new Node(value);
        if (!this.tail) {
            this.head = newNode;
            this.tail = this.head;
        } else {
            this.tail.next = newNode;
            this.tail = newNode;
        }
        this.length++;
    }
    prepend(value) {
        const newNode = new Node(value);
        if (!this.head) {
            this.head = newNode;
            this.tail = this.head;
        } else {
            newNode.next = this.head;
            this.head = newNode;
        }
        this.length++;
    }

    insert(index, value) {
        if (index >= this.length) {
            this.append(value);
            return;
        }
        if (index <= 0) {
            this.prepend(value);
            return;
        }
        const newNode = new Node(value);
        const currNode = this._traverse(index - 1);
        newNode.next = currNode.next;
        currNode.next = newNode;
        this.length++;
    }

    remove(index) {
        if (this.length === 0) {
            return null;
        }
        if (index <= 0) {
            // If index is 0, update the head to the next node
            this.head = this.head.next;
            this.length--;
            return;
        }
        if (index >= this.length) {
            index = this.length - 1;
        }
        const currNode = this._traverse(index - 1);
        currNode.next = currNode.next.next;
        this.length--;
    }

    reverse() {
        let prevNode = null;
        let currNode = this.head;
        while (currNode) {
            let nextNode = currNode.next;
            currNode.next = prevNode;
            prevNode = currNode;
            currNode = nextNode;
        }
        this.tail = this.head;
        this.head = prevNode;
    }
}

const list = new LinkedList();

// Test prepend function
console.log("Testing prepend function:");
list.prepend(10);
console.log("List after prepending 10:", list.toArray());

// Test append function
console.log("\nTesting append function:");
list.append(1);
console.log("List after appending 1:", list.toArray());

// Test insert function
console.log("\nTesting insert function:");
list.insert(2, 0);
console.log("List after inserting 2 at index 0:", list.toArray());

// Test insert function with out of bounds index
console.log("\nTesting insert function with out of bounds index:");
list.insert(11000, 10);
console.log("List after inserting 11000 at index 10:", list.toArray());

// Test reverse function
console.log("\nTesting reverse function:");
list.reverse();
console.log("List be reversed", list.toArray());

// Test remove function
console.log("\nTesting remove function:");
list.remove(2);
console.log("List after removing element at index 2:", list.toArray());

// Test remove function with out of bounds index
console.log("\nTesting remove function with out of bounds index:");
list.remove(10);
console.log(
    "List after attempting to remove element at index 10:",
    list.toArray(),
);

// Test remove function when list is empty
console.log("\nTesting remove function when list is empty:");
list.remove(0);
console.log(
    "List after attempting to remove element from empty list:",
    list.toArray(),
);

// Test remove function on single element list
console.log("\nTesting remove function on single element list:");
list.append(5);
list.remove(0);
console.log("List after removing single element:", list.toArray());
