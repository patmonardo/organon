// Define the Quality class
class Quality {
    private name: string;
    private isDeterminable: boolean;

    constructor(name: string, isDeterminable: boolean) {
        this.name = name;
        this.isDeterminable = isDeterminable;
    }

    public getName(): string {
        return this.name;
    }

    public isDeterminableQuality(): boolean {
        return this.isDeterminable;
    }

    public describe(): string {
        return `Quality: ${this.name}, Determinable: ${this.isDeterminable}`;
    }
}

// Define the Existence class
class Existence {
    private qualities: Quality[];
    private isImmediate: boolean;

    constructor(isImmediate: boolean) {
        this.qualities = [];
        this.isImmediate = isImmediate;
    }

    public addQuality(quality: Quality): void {
        this.qualities.push(quality);
    }

    public getQualities(): Quality[] {
        return this.qualities;
    }

    public isImmediateExistence(): boolean {
        return this.isImmediate;
    }

    public describe(): string {
        const qualitiesDescription = this.qualities.map(q => q.describe()).join(", ");
        return `Existence: Immediate: ${this.isImmediate}, Qualities: [${qualitiesDescription}]`;
    }
}

// Define the BeingForSelf class
class BeingForSelf {
    private existence: Existence;
    private selfReference: boolean;

    constructor(existence: Existence, selfReference: boolean) {
        this.existence = existence;
        this.selfReference = selfReference;
    }

    public isSelfReferential(): boolean {
        return this.selfReference;
    }

    public describe(): string {
        return `Being-for-Self: Self-Referential: ${this.selfReference}, Existence: [${this.existence.describe()}]`;
    }
}

// Example usage
const quality1 = new Quality("Indeterminateness", false);
const quality2 = new Quality("Immediacy", true);
const existence = new Existence(true);
existence.addQuality(quality1);
existence.addQuality(quality2);

const beingForSelf = new BeingForSelf(existence, true);

console.log(quality1.describe());
console.log(quality2.describe());
console.log(existence.describe());
console.log(beingForSelf.describe());