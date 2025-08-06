// Define an interface for common properties and methods
interface IConcept {
    description: string;
    isEqualTo(other: IConcept): boolean;
}

// Class representing the concept of Being
class Being implements IConcept {
    description: string;

    constructor() {
        this.description = "Pure being, without further determination. It is indeterminate and immediate.";
    }

    isEqualTo(other: IConcept): boolean {
        return other instanceof Being;
    }

    toString(): string {
        return `Being: ${this.description}`;
    }
}

// Class representing the concept of Existence
class Existence implements IConcept {
    description: string;
    quality: Quality;

    constructor(quality: Quality) {
        this.quality = quality;
        this.description = "Existence is the determinate being that arises from the unity of being and nothing.";
    }

    isEqualTo(other: IConcept): boolean {
        return other instanceof Existence && this.quality.isEqualTo((other as Existence).quality);
    }

    toString(): string {
        return `Existence: ${this.description}, Quality: ${this.quality.toString()}`;
    }
}

// Class representing the concept of Quality
class Quality implements IConcept {
    description: string;

    constructor() {
        this.description = "Quality is the immediate determinateness of being, representing its characteristics.";
    }

    isEqualTo(other: IConcept): boolean {
        return other instanceof Quality;
    }

    toString(): string {
        return `Quality: ${this.description}`;
    }
}

// Class representing the concept of Being-for-Self
class BeingForSelf implements IConcept {
    description: string;

    constructor() {
        this.description = "Being-for-Self is the self-referential aspect of existence, where being recognizes itself.";
    }

    isEqualTo(other: IConcept): boolean {
        return other instanceof BeingForSelf;
    }

    toString(): string {
        return `Being-for-Self: ${this.description}`;
    }
}

// Example usage
const pureBeing = new Being();
const quality = new Quality();
const existence = new Existence(quality);
const beingForSelf = new BeingForSelf();

console.log(pureBeing.toString());
console.log(quality.toString());
console.log(existence.toString());
console.log(beingForSelf.toString());

// Check equality
console.log(`Is existence equal to being? ${existence.isEqualTo(pureBeing)}`);
console.log(`Is quality equal to itself? ${quality.isEqualTo(new Quality())}`);