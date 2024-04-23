class HashTable:
    def __init__(self, size):
        self.data = [None] * size

    def __hash(self, key):
        hash_val = 0
        for i in range(len(key)):
            hash_val = (hash_val + ord(key[i]) * i) % len(self.data)
        return hash_val

    def set(self, key, value):
        key_hash = self.__hash(key)
        if not self.data[key_hash]:
            self.data[key_hash] = []
        self.data[key_hash].append((key, value))
        print(self.data)

    def get(self, key):
        key_hash = self.__hash(key)
        current_bucket = self.data[key_hash]
        if current_bucket:
            for item in current_bucket:
                if item[0] == key:
                    return item[1]
        return None

    def keys(self):
        if not self.data:
            return None
        result = []
        for item in self.data:
            if item:
                if len(self.data) > 1:
                    for sub_item in item:
                        result.append(sub_item[0])
                else:
                    result.append(item[0][0])
        return result


myHashTable = HashTable(2)

myHashTable.set("grapes", 10000)
myHashTable.set("apples", 9)

print(myHashTable.get("apples"))
print(myHashTable.get("grapes"))
