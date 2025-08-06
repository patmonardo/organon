// Define an interface for common properties and methods
interface IConcept {
    description: string;
    getDescription(): string;
}

// Class representing the concept of Quality
class Quality implements IConcept {
    description: string;
    private determinateness: string;

    constructor(determinateness: string) {
        this.determinateness = determinateness;
        this.description = `Quality is characterized by its determinateness: ${this.determinateness}.`;
    }

    getDescription(): string {
        return this.description;
    }

    // Method to demonstrate the relationship with existence
    relateToExistence(existence: Existence): string {
        return `Quality is determined in relation to existence: ${existence.getDescription()}`;
    }
}

// Class representing the concept of Existence
class Existence implements IConcept {
    description: string;
    private isIndeterminate: boolean;

    constructor(isIndeterminate: boolean) {
        this.isIndeterminate = isIndeterminate;
        this.description = `Existence is ${this.isIndeterminate ? 'indeterminate' : 'determinate'}.`;
    }

    getDescription(): string {
        return this.description;
    }

    // Method to demonstrate the relationship with quality
    relateToQuality(quality: Quality): string {
        return `Existence is defined through quality: ${quality.getDescription()}`;
    }
}

// Class representing the concept of Being-for-Self
class BeingForSelf implements IConcept {
    description: string;
    private selfReference: boolean;

    constructor(selfReference: boolean) {
        this.selfReference = selfReference;
        this.description = `Being-for-Self is characterized by self-reference: ${this.selfReference}.`;
    }

    getDescription(): string {
        return this.description;
    }

    // Method to demonstrate the relationship with existence
    relateToExistence(existence: Existence): string {
        return `Being-for-Self exists in relation to existence: ${existence.getDescription()}`;
    }
}

// Example usage
const quality = new Quality("the immediate determinateness of being");
const existence = new Existence(true);
const beingForSelf = new BeingForSelf(true);

console.log(quality.getDescription());
console.log(existence.getDescription());
console.log(beingForSelf.getDescription());

console.log(quality.relateToExistence(existence));
console.log(existence.relateToQuality(quality));
console.log(beingForSelf.relateToExistence(existence));