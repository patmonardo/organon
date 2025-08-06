// Define the base class for all concepts
abstract class Concept {
    abstract description(): string;
}

// Class representing Quality
class Quality extends Concept {
    private determinateness: string;
    private isImmediate: boolean;

    constructor(determinateness: string, isImmediate: boolean) {
        super();
        this.determinateness = determinateness;
        this.isImmediate = isImmediate;
    }

    description(): string {
        return `Quality: Determinateness is ${this.determinateness}, Immediate: ${this.isImmediate}`;
    }

    // Additional methods related to Quality can be added here
}

// Class representing Existence
class Existence extends Concept {
    private isDeterminate: boolean;
    private quality: Quality;

    constructor(isDeterminate: boolean, quality: Quality) {
        super();
        this.isDeterminate = isDeterminate;
        this.quality = quality;
    }

    description(): string {
        return `Existence: Determinate: ${this.isDeterminate}, Quality: [${this.quality.description()}]`;
    }

    // Additional methods related to Existence can be added here
}

// Class representing Being-for-Self
class BeingForSelf extends Concept {
    private existence: Existence;

    constructor(existence: Existence) {
        super();
        this.existence = existence;
    }

    description(): string {
        return `Being-for-Self: Existence: [${this.existence.description()}]`;
    }

    // Additional methods related to Being-for-Self can be added here
}

// Example usage
const quality = new Quality("Indeterminate", true);
const existence = new Existence(true, quality);
const beingForSelf = new BeingForSelf(existence);

console.log(quality.description());
console.log(existence.description());
console.log(beingForSelf.description());