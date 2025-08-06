// Define an interface for common properties and methods
interface IQuality {
    getDescription(): string;
}

// Class representing the concept of Quality
class Quality implements IQuality {
    private name: string;
    private characteristics: string[];

    constructor(name: string, characteristics: string[]) {
        this.name = name;
        this.characteristics = characteristics;
    }

    getDescription(): string {
        return `Quality: ${this.name}, Characteristics: ${this.characteristics.join(', ')}`;
    }
}

// Class representing the concept of Existence
class Existence {
    private quality: Quality;
    private isReal: boolean;

    constructor(quality: Quality, isReal: boolean) {
        this.quality = quality;
        this.isReal = isReal;
    }

    describeExistence(): string {
        return `Existence is characterized by ${this.quality.getDescription()} and is ${this.isReal ? 'real' : 'not real'}.`;
    }
}

// Class representing the concept of Being-for-Self
class BeingForSelf {
    private existence: Existence;

    constructor(existence: Existence) {
        this.existence = existence;
    }

    selfReference(): string {
        return `Being-for-Self recognizes its own existence: ${this.existence.describeExistence()}`;
    }
}

// Example usage
const quality = new Quality("Indeterminateness", ["immediate", "simple", "empty"]);
const existence = new Existence(quality, true);
const beingForSelf = new BeingForSelf(existence);

console.log(beingForSelf.selfReference());