// Define an interface for common properties and methods
interface IConcept {
    description: string;
    isEqualTo(other: IConcept): boolean;
}

// Class representing Quality
class Quality implements IConcept {
    description: string;
    private _determinateness: string;

    constructor(determinateness: string) {
        this._determinateness = determinateness;
        this.description = `Quality characterized by ${this._determinateness}.`;
    }

    isEqualTo(other: IConcept): boolean {
        return other instanceof Quality && this._determinateness === other._determinateness;
    }

    getDeterminateness(): string {
        return this._determinateness;
    }
}

// Class representing Existence
class Existence implements IConcept {
    description: string;
    private _quality: Quality;

    constructor(quality: Quality) {
        this._quality = quality;
        this.description = `Existence that embodies the quality of ${this._quality.getDeterminateness()}.`;
    }

    isEqualTo(other: IConcept): boolean {
        return other instanceof Existence && this._quality.isEqualTo(other._quality);
    }

    getQuality(): Quality {
        return this._quality;
    }
}

// Class representing Being-for-Self
class BeingForSelf implements IConcept {
    description: string;
    private _existence: Existence;

    constructor(existence: Existence) {
        this._existence = existence;
        this.description = `Being-for-Self that recognizes its existence as ${this._existence.description}.`;
    }

    isEqualTo(other: IConcept): boolean {
        return other instanceof BeingForSelf && this._existence.isEqualTo(other._existence);
    }

    getExistence(): Existence {
        return this._existence;
    }
}

// Example usage
const quality = new Quality("indeterminateness and immediacy");
const existence = new Existence(quality);
const beingForSelf = new BeingForSelf(existence);

console.log(quality.description); // Quality characterized by indeterminateness and immediacy.
console.log(existence.description); // Existence that embodies the quality of indeterminateness and immediacy.
console.log(beingForSelf.description); // Being-for-Self that recognizes its existence as Existence that embodies the quality of indeterminateness and immediacy.