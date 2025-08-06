// Define an interface for common properties and methods
interface IConcept {
    description: string;
    isEqualTo(other: IConcept): boolean;
}

// Class representing the concept of Quality
class Quality implements IConcept {
    description: string;
    private characteristics: string[];

    constructor(description: string, characteristics: string[]) {
        this.description = description;
        this.characteristics = characteristics;
    }

    // Method to check equality based on description
    isEqualTo(other: IConcept): boolean {
        return this.description === other.description;
    }

    // Method to get characteristics of the quality
    getCharacteristics(): string[] {
        return this.characteristics;
    }
}

// Class representing the concept of Existence
class Existence implements IConcept {
    description: string;
    private qualities: Quality[];

    constructor(description: string, qualities: Quality[]) {
        this.description = description;
        this.qualities = qualities;
    }

    // Method to check equality based on description
    isEqualTo(other: IConcept): boolean {
        return this.description === other.description;
    }

    // Method to get qualities of the existence
    getQualities(): Quality[] {
        return this.qualities;
    }

    // Method to describe the existence
    describe(): string {
        return `Existence: ${this.description}, Qualities: ${this.qualities.map(q => q.description).join(", ")}`;
    }
}

// Class representing the concept of Being-for-Self
class BeingForSelf implements IConcept {
    description: string;
    private existence: Existence;

    constructor(description: string, existence: Existence) {
        this.description = description;
        this.existence = existence;
    }

    // Method to check equality based on description
    isEqualTo(other: IConcept): boolean {
        return this.description === other.description;
    }

    // Method to describe the Being-for-Self
    describe(): string {
        return `Being-for-Self: ${this.description}, Existence: ${this.existence.describe()}`;
    }
}

// Example usage
const quality1 = new Quality("Indeterminateness", ["No specific characteristics", "Immediate", "Simple"]);
const quality2 = new Quality("Immediacy", ["Direct", "Unmediated"]);
const existence = new Existence("Determinate Being", [quality1, quality2]);
const beingForSelf = new BeingForSelf("Self-Referential Existence", existence);

console.log(beingForSelf.describe());