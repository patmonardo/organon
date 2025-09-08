export declare const LOGOS_NAMESPACE = "reality:logos";
export declare const LOGOS_SCOPE: 'being-only';
export declare const INVARIANTS: {
    throughOneAnother: string;
    firstPrinciple: string;
};
export declare const METHOD_SPEC: {
    chunks: {
        id: string;
        title: string;
        summary: string;
        source: string;
    }[];
    hlos: ({
        id: string;
        chunkId: string;
        label: string;
        digest: string;
        clauses: string[];
        witnessEdges?: undefined;
    } | {
        id: string;
        chunkId: string;
        label: string;
        witnessEdges: {
            type: string;
            from: string;
            to: string;
        }[];
        digest?: undefined;
        clauses?: undefined;
    })[];
};
export declare const CHUNKS: {
    id: string;
    title: string;
    summary: string;
    source: string;
}[];
export declare const HLOS: ({
    id: string;
    chunkId: string;
    label: string;
    digest: string;
    clauses: string[];
    witnessEdges?: undefined;
} | {
    id: string;
    chunkId: string;
    label: string;
    witnessEdges: {
        type: string;
        from: string;
        to: string;
    }[];
    digest?: undefined;
    clauses?: undefined;
})[];
