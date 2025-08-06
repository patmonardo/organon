// Define the base class for Being
class Being {
    protected quality: Quality;
    protected existence: Existence;

    constructor(quality: Quality, existence: Existence) {
        this.quality = quality;
        this.existence = existence;
    }

    public describe(): string {
        return `Being with quality: ${this.quality.describe()} and existence: ${this.existence.describe()}`;
    }
}

// Define the Quality class
class Quality {
    private determinateness: string;

    constructor(determinateness: string) {
        this.determinateness = determinateness;
    }

    public describe(): string {
        return `Quality of determinateness: ${this.determinateness}`;
    }

    public isEqualTo(other: Quality): boolean {
        return this.determinateness === other.determinateness;
    }
}

// Define the Existence class
class Existence {
    private state: string;

    constructor(state: string) {
        this.state = state;
    }

    public describe(): string {
        return `Existence in state: ${this.state}`;
    }

    public isSameAs(other: Existence): boolean {
        return this.state === other.state;
    }
}

// Define the BeingForSelf class
class BeingForSelf extends Being {
    private selfReference: string;

    constructor(quality: Quality, existence: Existence, selfReference: string) {
        super(quality, existence);
        this.selfReference = selfReference;
    }

    public describe(): string {
        return `${super.describe()} and Being-for-Self with self-reference: ${this.selfReference}`;
    }

    public reflectOnSelf(): string {
        return `Reflecting on self: ${this.selfReference}`;
    }
}

// Example usage
const quality = new Quality("Indeterminate and Immediate");
const existence = new Existence("Present and Concrete");
const beingForSelf = new BeingForSelf(quality, existence, "I am");

console.log(beingForSelf.describe());
console.log(beingForSelf.reflectOnSelf());