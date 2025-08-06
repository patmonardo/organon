// Define an interface for common properties and methods
interface IConcept {
    description: string;
    isEqualTo(other: IConcept): boolean;
}

// Class representing the concept of Quality
class Quality implements IConcept {
    description: string;
    private determinateness: string;

    constructor(determinateness: string) {
        this.determinateness = determinateness;
        this.description = `Quality characterized by ${this.determinateness}.`;
    }

    isEqualTo(other: IConcept): boolean {
        return other instanceof Quality && this.determinateness === other.determinateness;
    }

    getDeterminateness(): string {
        return this.determinateness;
    }
}

// Class representing the concept of Existence
class Existence implements IConcept {
    description: string;
    private quality: Quality;

    constructor(quality: Quality) {
        this.quality = quality;
        this.description = `Existence that embodies the quality of ${this.quality.getDeterminateness()}.`;
    }

    isEqualTo(other: IConcept): boolean {
        return other instanceof Existence && this.quality.isEqualTo(other.getQuality());
    }

    getQuality(): Quality {
        return this.quality;
    }
}

// Class representing the concept of Being-for-Self
class BeingForSelf implements IConcept {
    description: string;
    private existence: Existence;

    constructor(existence: Existence) {
        this.existence = existence;
        this.description = `Being-for-Self that recognizes its own existence as ${this.existence.description}.`;
    }

    isEqualTo(other: IConcept): boolean {
        return other instanceof BeingForSelf && this.existence.isEqualTo(other.getExistence());
    }

    getExistence(): Existence {
        return this.existence;
    }
}

// Example usage
const quality = new Quality("indeterminateness and immediacy");
const existence = new Existence(quality);
const beingForSelf = new BeingForSelf(existence);

console.log(quality.description); // Quality characterized by indeterminateness and immediacy.
console.log(existence.description); // Existence that embodies the quality of indeterminateness and immediacy.
console.log(beingForSelf.description); // Being-for-Self that recognizes its own existence as Existence that embodies the quality of indeterminateness and immediacy.