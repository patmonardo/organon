// Define the base class for Being
class Being {
    protected isEqualTo: boolean;
    protected hasNoDifference: boolean;

    constructor() {
        this.isEqualTo = true; // Being is equal only to itself
        this.hasNoDifference = true; // Being has no internal or external differences
    }

    describe(): string {
        return "Being is pure, indeterminate, and immediate. It has no specific characteristics.";
    }
}

// Define the Quality class extending Being
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

    describe(): string {
        return `Quality is characterized by:
        - Indeterminateness: ${this.indeterminateness}
        - Immediacy: ${this.immediacy}
        - Emptiness: ${this.emptiness}`;
    }
}

// Define the Existence class extending Being
class Existence extends Being {
    private quality: Quality;

    constructor(quality: Quality) {
        super();
        this.quality = quality;
    }

    describe(): string {
        return `Existence is the manifestation of being with quality. It embodies:
        - ${this.quality.describe()}`;
    }
}

// Define the BeingForSelf class extending Being
class BeingForSelf extends Being {
    private selfReference: boolean;

    constructor() {
        super();
        this.selfReference = true; // Being-for-Self is self-referential
    }

    describe(): string {
        return `Being-for-Self is characterized by self-reference and self-determination. It is the realization of being as an individual entity.`;
    }
}

// Example usage
const quality = new Quality();
const existence = new Existence(quality);
const beingForSelf = new BeingForSelf();

console.log(quality.describe());
console.log(existence.describe());
console.log(beingForSelf.describe());