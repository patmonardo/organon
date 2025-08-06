// Define an interface for the basic properties of a Being
interface IBeing {
    isEqualTo(other: IBeing): boolean;
    hasNoDifference(other: IBeing): boolean;
}

// Class representing the concept of Being
class Being implements IBeing {
    private characteristics: string[];

    constructor() {
        this.characteristics = ['Indeterminateness', 'Immediacy', 'Emptiness'];
    }

    // Method to check if two beings are equal
    isEqualTo(other: IBeing): boolean {
        return this === other; // Simplified equality check
    }

    // Method to check if there is no difference between two beings
    hasNoDifference(other: IBeing): boolean {
        return this.isEqualTo(other);
    }

    // Method to get characteristics of Being
    getCharacteristics(): string[] {
        return this.characteristics;
    }
}

// Class representing the concept of Quality
class Quality extends Being {
    private quality: string;

    constructor(quality: string) {
        super();
        this.quality = quality;
    }

    // Method to get the quality
    getQuality(): string {
        return this.quality;
    }

    // Method to describe the quality
    describeQuality(): string {
        return `Quality: ${this.quality}`;
    }
}

// Class representing the concept of Being-for-Self
class BeingForSelf extends Being {
    private selfReference: string;

    constructor(selfReference: string) {
        super();
        this.selfReference = selfReference;
    }

    // Method to get self-reference
    getSelfReference(): string {
        return this.selfReference;
    }

    // Method to describe Being-for-Self
    describeBeingForSelf(): string {
        return `Being-for-Self: ${this.selfReference}`;
    }
}

// Example usage
const pureBeing = new Being();
console.log("Characteristics of Being:", pureBeing.getCharacteristics());

const qualityInstance = new Quality("Color");
console.log(qualityInstance.describeQuality());

const beingForSelfInstance = new BeingForSelf("I am conscious");
console.log(beingForSelfInstance.describeBeingForSelf());