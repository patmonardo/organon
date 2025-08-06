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
    private quality: Quality;

    constructor(description: string, quality: Quality) {
        this.description = description;
        this.quality = quality;
    }

    // Method to check equality based on description
    isEqualTo(other: IConcept): boolean {
        return this.description === other.description;
    }

    // Method to get the quality associated with existence
    getQuality(): Quality {
        return this.quality;
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

    // Method to get the existence associated with being-for-self
    getExistence(): Existence {
        return this.existence;
    }
}

// Example usage
const quality = new Quality("Quality of Being", ["Indeterminate", "Immediate", "Empty"]);
const existence = new Existence("Existence as Determinate Being", quality);
const beingForSelf = new BeingForSelf("Being-for-Self as Self-Determining Existence", existence);

// Output the descriptions and characteristics
console.log(`Quality: ${quality.description}, Characteristics: ${quality.getCharacteristics().join(", ")}`);
console.log(`Existence: ${existence.description}, Associated Quality: ${existence.getQuality().description}`);
console.log(`Being-for-Self: ${beingForSelf.description}, Associated Existence: ${beingForSelf.getExistence().description}`);