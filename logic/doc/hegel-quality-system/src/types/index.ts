// Define the base class for Being
class Being {
    protected isEqual: boolean;
    protected hasNoDifference: boolean;

    constructor() {
        this.isEqual = true; // Being is equal only to itself
        this.hasNoDifference = true; // Being has no internal or external differences
    }

    public describe(): string {
        return "Being is pure, indeterminate, and immediate. It is equal only to itself and has no differences.";
    }
}

// Define the class for Quality, extending Being
class Quality extends Being {
    private indeterminateness: boolean;
    private immediacy: boolean;
    private emptiness: boolean;

    constructor() {
        super();
        this.indeterminateness = true; // Quality is indeterminate
        this.immediacy = true; // Quality is immediate
        this.emptiness = true; // Quality is empty
    }

    public describe(): string {
        return `Quality is characterized by:
        - Indeterminateness: ${this.indeterminateness}
        - Immediacy: ${this.immediacy}
        - Emptiness: ${this.emptiness}`;
    }
}

// Define the class for Existence, extending Being
class Existence extends Being {
    private quality: Quality;

    constructor() {
        super();
        this.quality = new Quality(); // Existence has a quality
    }

    public describe(): string {
        return `Existence is the manifestation of being that includes:
        - Quality: ${this.quality.describe()}`;
    }
}

// Define the class for Being-for-Self, extending Being
class BeingForSelf extends Being {
    private selfReference: boolean;

    constructor() {
        super();
        this.selfReference = true; // Being-for-Self is self-referential
    }

    public describe(): string {
        return `Being-for-Self is characterized by self-reference:
        - Self-Reference: ${this.selfReference}`;
    }
}

// Example usage
const being = new Being();
console.log(being.describe());

const quality = new Quality();
console.log(quality.describe());

const existence = new Existence();
console.log(existence.describe());

const beingForSelf = new BeingForSelf();
console.log(beingForSelf.describe());