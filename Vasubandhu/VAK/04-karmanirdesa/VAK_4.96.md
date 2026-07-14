# VAK_4.96

## Sanskrit

आनन्तर्याणि कर्माणि तीव्रक्लेशोऽथ दुर्गतिः ।
कौरवासंज्ञित्वं च मतमावरणत्रयम् ॥

## IAST

ānantaryāṇi karmāṇi tīvrakleśo ’tha durgatiḥ |
kauravāsaṃjñitvaṃ ca matam āvaraṇatrayam ||

## Source caution

The source gives `kauravāsaṃjñittvāś ca`. The required sense is `kaurava + asaṃjñitva`: existence among the Kurus and the state of being non-percipient. Both belong with `durgatiḥ` under the third obstruction, the obstruction constituted by resultant existence.

## Padaccheda

ānantaryāṇi | karmāṇi | tīvra-kleśaḥ | atha | durgatiḥ | kaurava | asaṃjñitvam | ca | matam | āvaraṇa-trayam

Expanded:

ānantaryāṇi karmāṇi, tīvra-kleśaḥ, atha durgatiḥ kauravam asaṃjñitvaṃ ca—āvaraṇa-trayaṃ matam.

## Grammar

- `ānantaryāṇi`: nominative neuter plural, having immediate consequence
- `karmāṇi`: nominative neuter plural, karmic activities
- `tīvra-kleśaḥ`: nominative masculine singular, intense affliction
- `atha`: and then, further
- `durgatiḥ`: nominative feminine singular, bad destiny
- `kaurava`: belonging to or existing among the Kurus
- `asaṃjñitvam`: nominative neuter singular, non-percipient existence
- `ca`: and
- `matam`: held, regarded
- `āvaraṇa-trayam`: the threefold obstruction

`Āvaraṇa-trayam` organizes the several items under three kinds of obstruction.

## Literal translation

The karmic activities of immediate consequence, intense affliction, and then bad destiny, existence among the Kurus, and non-percipient existence are held to constitute the threefold obstruction.

## Philosophical translation

Three obstructions are recognized: karmic obstruction, consisting in the acts of immediate consequence; afflictive obstruction, consisting in intense affliction; and resultant obstruction, consisting in a bad destiny, birth among the Kurus, or non-percipient existence.

- `karmāvaraṇa`: obstruction through karma
- `kleśāvaraṇa`: obstruction through affliction
- `vipākāvaraṇa`: obstruction through karmic result

## Technical vocabulary

### Ānantarya-karma

`Ānantarya` means without an interval or immediately following. An `ānantarya-karma` is an act whose fully matured result follows without another birth intervening.

The five standard acts are killing one’s mother, killing one’s father, killing an arhat, causing the blood of a Tathāgata to flow with hostile intention, and division of the Saṃgha.

These acts exercise such determinative force over the karmic continuum that they obstruct the arising of the Path.

### Tīvra-kleśa

This is not merely the presence of affliction. It is affliction intensified to such a degree that the Path cannot presently arise.

### Durgati

The bad destinies are hell existence, preta existence, and animal existence. They are obstructions because these modes of birth do not provide the operative conditions required for entrance into the Path.

### Kaurava

This concerns beings born in Uttarakuru. Birth there is fortunate, yet the ease and abundance of that destiny prevent the urgency and discriminative orientation required for entering the Path.

Pleasant result is therefore not identical with a condition suitable for the Path.

### Asaṃjñitva

This is the state of non-percipient existence among the `asaṃjñi-sattvas`. Because manifest cognitive activity is absent, the discriminative operations necessary for entering the Path cannot occur.

## Doctrinal determination

The verse distinguishes three locations of obstruction:

- activity: karma
- afflictive condition: kleśa
- resultant mode of existence: vipāka

Thus:

- `karmāvaraṇa`: the continuum is obstructed by what has been done
- `kleśāvaraṇa`: the continuum is obstructed by its present afflictive constitution
- `vipākāvaraṇa`: the continuum is obstructed by the form of existence presently produced

The obstruction may lie behind the present state as determining activity, within the present state as dominant affliction, or as the present state itself as resultant embodiment or destiny.

## Logical determination

The three moments are:

- Karma: determining activity
- Kleśa: determining inward condition
- Vipāka: determined existence

These are not independent substances but three locations of obstruction within one karmic continuum. An obstruction is a determination of the continuum itself: what it has done, what presently governs it, and what it has consequently become.

## Interpretive note

VAK_4.95 explains that one karma projects one birth while many karmas complete it. VAK_4.96 now identifies determinations that do not merely project or complete existence but obstruct the Path within that existence.

The decisive distinction is therefore between karma as production of a world and karma as closure of a path.

The inclusion of Uttarakuru and non-percipient existence shows that the classification is not based upon pleasure and pain. The criterion is functional: can this continuum generate the Path?

## Structural schema

```text
āvaraṇa-traya
│
├── karmāvaraṇa
│   └── ānantarya-karmāṇi
│
├── kleśāvaraṇa
│   └── tīvra-kleśa
│
└── vipākāvaraṇa
    ├── durgati
    ├── kaurava
    └── asaṃjñitva
```

## OWL++ seed

```text
Class: Avarana
    determination: obstruction of Path-arising

Class: KarmaAvarana
    SubClassOf: Avarana
    groundedIn: AnantaryaKarma

Class: KlesaAvarana
    SubClassOf: Avarana
    groundedIn: TivraKlesa

Class: VipakaAvarana
    SubClassOf: Avarana
    groundedIn some:
        Durgati
        or KauravaExistence
        or Asamjnitva

Class: AnantaryaKarma
    SubClassOf: Karma
    characteristic:
        immediateResultAfterDeath

Class: PathCompatibleExistence
Class: PathIncompatibleExistence

Durgati SubClassOf: PathIncompatibleExistence
KauravaExistence SubClassOf: PathIncompatibleExistence
Asamjnitva SubClassOf: PathIncompatibleExistence
```

Critical relation:

```text
obstructs
    domain: karmic or resultant determination
    range: arising of the Path
```

## Commit note

VAK_4.96 — establish the three obstructions

- identify karmāvaraṇa with ānantarya karma
- identify kleśāvaraṇa with intense affliction
- identify vipākāvaraṇa with bad destinies, Uttarakuru birth, and non-percipient existence
- distinguish unpleasant result from Path-incompatible result
- define obstruction as a determination internal to the continuum
