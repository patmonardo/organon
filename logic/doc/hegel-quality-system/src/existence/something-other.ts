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
    private isImmediate: boolean;

    constructor(determinateness: string, isImmediate: boolean) {
        this.determinateness = determinateness;
        this.isImmediate = isImmediate;
    }

    public describe(): string {
        return `Quality - Determinateness: ${this.determinateness}, Immediate: ${this.isImmediate}`;
    }

    public isQualityImmediate(): boolean {
        return this.isImmediate;
    }
}

// Define the Existence class
class Existence {
    private isConcrete: boolean;
    private isDeterminate: boolean;

    constructor(isConcrete: boolean, isDeterminate: boolean) {
        this.isConcrete = isConcrete;
        this.isDeterminate = isDeterminate;
    }

    public describe(): string {
        return `Existence - Concrete: ${this.isConcrete}, Determinate: ${this.isDeterminate}`;
    }

    public isExistenceConcrete(): boolean {
        return this.isConcrete;
    }
}

// Define the BeingForSelf class
class BeingForSelf extends Being {
    private selfReference: boolean;

    constructor(quality: Quality, existence: Existence, selfReference: boolean) {
        super(quality, existence);
        this.selfReference = selfReference;
    }

    public describe(): string {
        return `${super.describe()}, Being-for-Self - Self-Reference: ${this.selfReference}`;
    }

    public isSelfReferential(): boolean {
        return this.selfReference;
    }
}

// Example usage
const quality = new Quality("Indeterminate", true);
const existence = new Existence(true, true);
const beingForSelf = new BeingForSelf(quality, existence, true);

console.log(beingForSelf.describe());