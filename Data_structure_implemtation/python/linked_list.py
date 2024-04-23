class Node:

    def __init__(self, value):
        self.value = value
        self.next = None


class LinkedList:

    def __init__(self):
        self.head = None
        self.tail = None
        self.length = 0

    def __traverse(self, index):
        curr_node = self.head
        i = 0
        while i < index:
            curr_node = curr_node.next
            i += 1
        return curr_node

    def to_array(self):
        array = []
        curr_node = self.head
        while curr_node:
            array.append(curr_node.value)
            curr_node = curr_node.next
        return array

    def append(self, value):
        new_node = Node(value)
        if not self.tail:
            self.head = new_node
            self.tail = self.head
        else:
            self.tail.next = new_node
            self.tail = new_node
        self.length += 1

    def prepend(self, value):
        new_node = Node(value)
        if not self.head:
            self.head = new_node
            self.tail = self.head
        else:
            new_node.next = self.head
            self.head = new_node
        self.length += 1

    def insert(self, index, value):
        if index >= self.length:
            self.append(value)
            return
        if index <= 0:
            self.prepend(value)
            return
        new_node = Node(value)
        prev_node = self.__traverse(index - 1)
        new_node.next = prev_node.next
        prev_node.next = new_node
        self.length += 1

    def remove(self, index):
        if self.length == 0:
            return None
        if index <= 0:
            self.head = self.head.next
            self.length -= 1
            if self.length == 0:
                self.tail = None
            return
        if index >= self.length:
            index = self.length - 1
        prev_node = self.__traverse(index - 1)
        prev_node.next = prev_node.next.next
        self.length -= 1
        if index == self.length - 1:
            self.tail = prev_node

    def reverse(self):
        prev_node = None
        curr_node = self.head
        while curr_node:
            next_node = curr_node.next
            curr_node.next = prev_node
            prev_node = curr_node
            curr_node = next_node
        self.tail = self.head
        self.head = prev_node


# Test the LinkedList implementation
list = LinkedList()

# Test prepend function
print("Testing prepend function:")
list.prepend(10)
print("List after prepending 10:", list.to_array())

# Test append function
print("\nTesting append function:")
list.append(1)
print("List after appending 1:", list.to_array())

# Test insert function
print("\nTesting insert function:")
list.insert(2, 0)
print("List after inserting 2 at index 0:", list.to_array())

# Test insert function with out of bounds index
print("\nTesting insert function with out of bounds index:")
list.insert(11000, 10)
print("List after inserting 11000 at index 10:", list.to_array())

# Test reverse function
print("\nTesting reverse function:")
list.reverse()
print("List be reversed", list.to_array())

# Test remove function
print("\nTesting remove function:")
list.remove(2)
print("List after removing element at index 2:", list.to_array())

# Test remove function with out of bounds index
print("\nTesting remove function with out of bounds index:")
list.remove(10)
print("List after attempting to remove element at index 10:", list.to_array())

# Test remove function when list is empty
print("\nTesting remove function when list is empty:")
list.remove(0)
print("List after attempting to remove element from empty list:", list.to_array())

# Test remove function on single element list
print("\nTesting remove function on single element list:")
# list.append(5)
list.remove(0)
print("List after removing single element:", list.to_array())

