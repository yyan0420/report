/**
 * @generated SignedSource<<54e4ced49703f193cfc4a9aeefed29ef>>
 * @lightSyntaxTransform
 * @nogrep
 */

/* tslint:disable */
/* eslint-disable */
// @ts-nocheck

import type { ConcreteRequest } from 'relay-runtime';
export type BrandQuery$variables = Record<PropertyKey, never>;
export type BrandQuery$data = {
  readonly yoyTable: ReadonlyArray<{
    readonly code: string | null | undefined;
    readonly name: string;
    readonly percentage1: number | null | undefined;
    readonly percentage2: number | null | undefined;
    readonly percentage3: number | null | undefined;
    readonly qty1: number;
    readonly qty2: number;
    readonly qty3: number;
    readonly qty4: number;
    readonly total1: number;
    readonly total2: number;
    readonly total3: number;
    readonly total4: number;
    readonly totalDiff1: number;
    readonly totalDiff2: number;
    readonly totalDiff3: number;
  }>;
};
export type BrandQuery = {
  response: BrandQuery$data;
  variables: BrandQuery$variables;
};

const node: ConcreteRequest = (function(){
var v0 = [
  {
    "alias": null,
    "args": null,
    "concreteType": "Summary",
    "kind": "LinkedField",
    "name": "yoyTable",
    "plural": true,
    "selections": [
      {
        "alias": null,
        "args": null,
        "kind": "ScalarField",
        "name": "name",
        "storageKey": null
      },
      {
        "alias": null,
        "args": null,
        "kind": "ScalarField",
        "name": "code",
        "storageKey": null
      },
      {
        "alias": null,
        "args": null,
        "kind": "ScalarField",
        "name": "qty1",
        "storageKey": null
      },
      {
        "alias": null,
        "args": null,
        "kind": "ScalarField",
        "name": "total1",
        "storageKey": null
      },
      {
        "alias": null,
        "args": null,
        "kind": "ScalarField",
        "name": "qty2",
        "storageKey": null
      },
      {
        "alias": null,
        "args": null,
        "kind": "ScalarField",
        "name": "total2",
        "storageKey": null
      },
      {
        "alias": null,
        "args": null,
        "kind": "ScalarField",
        "name": "qty3",
        "storageKey": null
      },
      {
        "alias": null,
        "args": null,
        "kind": "ScalarField",
        "name": "total3",
        "storageKey": null
      },
      {
        "alias": null,
        "args": null,
        "kind": "ScalarField",
        "name": "qty4",
        "storageKey": null
      },
      {
        "alias": null,
        "args": null,
        "kind": "ScalarField",
        "name": "total4",
        "storageKey": null
      },
      {
        "alias": null,
        "args": null,
        "kind": "ScalarField",
        "name": "percentage1",
        "storageKey": null
      },
      {
        "alias": null,
        "args": null,
        "kind": "ScalarField",
        "name": "percentage2",
        "storageKey": null
      },
      {
        "alias": null,
        "args": null,
        "kind": "ScalarField",
        "name": "percentage3",
        "storageKey": null
      },
      {
        "alias": null,
        "args": null,
        "kind": "ScalarField",
        "name": "totalDiff1",
        "storageKey": null
      },
      {
        "alias": null,
        "args": null,
        "kind": "ScalarField",
        "name": "totalDiff2",
        "storageKey": null
      },
      {
        "alias": null,
        "args": null,
        "kind": "ScalarField",
        "name": "totalDiff3",
        "storageKey": null
      }
    ],
    "storageKey": null
  }
];
return {
  "fragment": {
    "argumentDefinitions": [],
    "kind": "Fragment",
    "metadata": null,
    "name": "BrandQuery",
    "selections": (v0/*: any*/),
    "type": "QueryRoot",
    "abstractKey": null
  },
  "kind": "Request",
  "operation": {
    "argumentDefinitions": [],
    "kind": "Operation",
    "name": "BrandQuery",
    "selections": (v0/*: any*/)
  },
  "params": {
    "cacheID": "a073ab95901bc1acc6247292b6db0f11",
    "id": null,
    "metadata": {},
    "name": "BrandQuery",
    "operationKind": "query",
    "text": "query BrandQuery {\n  yoyTable {\n    name\n    code\n    qty1\n    total1\n    qty2\n    total2\n    qty3\n    total3\n    qty4\n    total4\n    percentage1\n    percentage2\n    percentage3\n    totalDiff1\n    totalDiff2\n    totalDiff3\n  }\n}\n"
  }
};
})();

(node as any).hash = "73777d1491f4c69bf3edf9fc76280f3b";

export default node;
