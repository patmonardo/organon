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

    // Method to display information about the quality
    displayInfo(): void {
        console.log(`Quality: ${this.description}`);
        console.log(`Characteristics: ${this.characteristics.join(', ')}`);
    }
}

// Class representing the concept of Existence
class Existence implements IConcept {
    description: string;
    private qualities: Quality[];

    constructor(description: string) {
        this.description = description;
        this.qualities = [];
    }

    // Method to add a quality to existence
    addQuality(quality: Quality): void {
        this.qualities.push(quality);
    }

    // Method to check equality based on description
    isEqualTo(other: IConcept): boolean {
        return this.description === other.description;
    }

    // Method to display information about existence
    displayInfo(): void {
        console.log(`Existence: ${this.description}`);
        console.log(`Qualities: ${this.qualities.map(q => q.description).join(', ')}`);
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

    // Method to display information about being-for-self
    displayInfo(): void {
        console.log(`Being-for-Self: ${this.description}`);
        console.log(`Related Existence: ${this.existence.description}`);
    }
}

// Example usage
const quality1 = new Quality("Indeterminateness", ["No specific qualities", "Immediate", "Simple"]);
const quality2 = new Quality("Immediacy", ["Direct", "Unmediated"]);

const existence = new Existence("Existence as a determinate being");
existence.addQuality(quality1);
existence.addQuality(quality2);

const beingForSelf = new BeingForSelf("Being-for-Self", existence);

// Display information
quality1.displayInfo();
quality2.displayInfo();
existence.displayInfo();
beingForSelf.displayInfo();