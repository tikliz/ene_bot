export class ArrayWHistory<T> {
    private array: T[];
    private history: T[][];
    private storageKey: string;
  
    constructor(initialArray: T[], storageKey: string) {
      this.array = initialArray;
      this.storageKey = storageKey;
      this.history = [initialArray.slice()];

    }
  
    get length() {
      return this.array.length;

    }
  
    replace(newArray: T[]) {
        this.history.push(this.array.slice());
        this.array = newArray;
        this.saveHistory();

    }

    push(...items: T[]) {
      console.log("pushed: ", items);
        this.array.push(...items);
      this.saveHistory();

    }
  
    splice(index) {
      let temp = this.array.splice(index, 1);
      console.log("spliced: ", temp);
      this.saveHistory();

    }

    pop() {
        this.array.pop();
        this.saveHistory();

    }
  
    undo() {
      if (this.history.length > 1) {
        this.array = this.history[this.history.length - 2].slice();
        this.saveHistory();

      }
    }
  
    toArray() {
      return this.array.slice();

    }
  
    private saveHistory() {
    //   localStorage.setItem(this.storageKey, JSON.stringify(this.history));
      this.history.push(this.array.slice());

    }


  }
  