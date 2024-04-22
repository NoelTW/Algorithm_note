class HashTable {
    constructor(size) {
        this.data = new Array(size);
    }

    _hash(key) {
        let hash = 0;
        for (let i = 0; i < key.length; i++) {
            hash = (hash + key.charCodeAt(i) * i) % this.data.length;
        }
        return hash;
    }

    set(key, value) {
        let key_hash = this._hash(key);
        if (!this.data[key_hash]) {
            this.data[key_hash] = [];
        }
        this.data[key_hash].push([key, value]);
        console.log(this.data);
    }
    get(key) {
        let key_hash = this._hash(key);
        let currentBucket = this.data[key_hash];
        if (currentBucket) {
            for (let item of currentBucket) {
                if (item[0] == key) {
                    return item[1];
                }
            }
        }
        return undefined;
    }
    keys() {
        if (!this.data.length) {
            return undefined;
        }
        let result = [];
        // loop through all the elements
        for (let i = 0; i < this.data.length; i++) {
            // if it's not an empty memory cell
            if (this.data[i] && this.data[i].length) {
                // but also loop through all the potential collisions
                if (this.data.length > 1) {
                    for (let j = 0; j < this.data[i].length; j++) {
                        result.push(this.data[i][j][0]);
                    }
                } else {
                    result.push(this.data[i][0]);
                }
            }
        }
        return result;
    }
}

const myHashTable = new HashTable(2);

myHashTable.set("grapes", 10000);
// console.log(myHashTable.get("grapes"));
myHashTable.set("apples", 9);
// console.log(myHashTable.get("apples"));

console.log(myHashTable.get("apples"));

console.log(myHashTable.get("grapes"));
