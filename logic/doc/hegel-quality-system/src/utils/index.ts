// Define the base class for Being
class Being {
    protected isEqualTo: boolean;
    protected hasNoDifference: boolean;

    constructor() {
        this.isEqualTo = true; // Being is equal only to itself
        this.hasNoDifference = true; // Being has no internal or external differences
    }

    public describe(): string {
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

    public describe(): string {
        return `Quality is characterized by:
        - Indeterminateness: ${this.indeterminateness}
        - Immediacy: ${this.immediacy}
        - Emptiness: ${this.emptiness}`;
    }
}

// Define the Existence class extending Being
class Existence extends Being {
    private quality: Quality;

    constructor() {
        super();
        this.quality = new Quality(); // Existence has a quality
    }

    public describe(): string {
        return `Existence is the manifestation of being with quality. It is characterized by:
        - Quality: ${this.quality.describe()}`;
    }
}

// Define the BeingForSelf class extending Being
class BeingForSelf extends Being {
    private existence: Existence;

    constructor() {
        super();
        this.existence = new Existence(); // Being-for-Self has existence
    }

    public describe(): string {
        return `Being-for-Self is the self-referential aspect of existence. It is characterized by:
        - Existence: ${this.existence.describe()}`;
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