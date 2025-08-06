// Define an interface for a general concept
interface Concept {
    description: string;
    isEqualTo(other: Concept): boolean;
}

// Class representing Quality
class Quality implements Concept {
    description: string;
    private characteristics: string[];

    constructor(description: string, characteristics: string[]) {
        this.description = description;
        this.characteristics = characteristics;
    }

    // Method to check equality based on description
    isEqualTo(other: Concept): boolean {
        return this.description === other.description;
    }

    // Method to display characteristics
    displayCharacteristics(): void {
        console.log(`Characteristics of ${this.description}: ${this.characteristics.join(', ')}`);
    }
}

// Class representing Existence
class Existence implements Concept {
    description: string;
    private quality: Quality;

    constructor(description: string, quality: Quality) {
        this.description = description;
        this.quality = quality;
    }

    // Method to check equality based on description
    isEqualTo(other: Concept): boolean {
        return this.description === other.description;
    }

    // Method to display the quality of existence
    displayQuality(): void {
        console.log(`Existence: ${this.description}`);
        this.quality.displayCharacteristics();
    }
}

// Class representing Being-for-Self
class BeingForSelf implements Concept {
    description: string;
    private existence: Existence;

    constructor(description: string, existence: Existence) {
        this.description = description;
        this.existence = existence;
    }

    // Method to check equality based on description
    isEqualTo(other: Concept): boolean {
        return this.description === other.description;
    }

    // Method to display the relationship with existence
    displayExistence(): void {
        console.log(`Being-for-Self: ${this.description}`);
        this.existence.displayQuality();
    }
}

// Example usage
const quality = new Quality("Quality of Pure Being", ["Indeterminateness", "Immediacy", "Emptiness"]);
const existence = new Existence("Existence as Determinate Being", quality);
const beingForSelf = new BeingForSelf("Being-for-Self as Self-Determining Existence", existence);

// Display the information
beingForSelf.displayExistence();