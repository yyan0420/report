/**
 * @generated SignedSource<<93a304ce6428484aa9ab1248daaccb86>>
 * @lightSyntaxTransform
 * @nogrep
 */

/* tslint:disable */
/* eslint-disable */
// @ts-nocheck

import type { ConcreteRequest } from 'relay-runtime';
export type BrandListQuery$variables = Record<PropertyKey, never>;
export type BrandListQuery$data = {
  readonly brands: {
    readonly edges: ReadonlyArray<{
      readonly node: {
        readonly id: string;
        readonly name: string;
        readonly privateLabel: boolean;
        readonly status: boolean;
        readonly urlSlug: string;
      };
    }>;
  };
};
export type BrandListQuery = {
  response: BrandListQuery$data;
  variables: BrandListQuery$variables;
};

const node: ConcreteRequest = (function(){
var v0 = [
  {
    "alias": null,
    "args": [
      {
        "kind": "Literal",
        "name": "first",
        "value": 1000
      }
    ],
    "concreteType": "BrandConnection",
    "kind": "LinkedField",
    "name": "brands",
    "plural": false,
    "selections": [
      {
        "alias": null,
        "args": null,
        "concreteType": "BrandEdge",
        "kind": "LinkedField",
        "name": "edges",
        "plural": true,
        "selections": [
          {
            "alias": null,
            "args": null,
            "concreteType": "Brand",
            "kind": "LinkedField",
            "name": "node",
            "plural": false,
            "selections": [
              {
                "alias": null,
                "args": null,
                "kind": "ScalarField",
                "name": "id",
                "storageKey": null
              },
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
                "name": "urlSlug",
                "storageKey": null
              },
              {
                "alias": null,
                "args": null,
                "kind": "ScalarField",
                "name": "privateLabel",
                "storageKey": null
              },
              {
                "alias": null,
                "args": null,
                "kind": "ScalarField",
                "name": "status",
                "storageKey": null
              }
            ],
            "storageKey": null
          }
        ],
        "storageKey": null
      }
    ],
    "storageKey": "brands(first:1000)"
  }
];
return {
  "fragment": {
    "argumentDefinitions": [],
    "kind": "Fragment",
    "metadata": null,
    "name": "BrandListQuery",
    "selections": (v0/*: any*/),
    "type": "QueryRoot",
    "abstractKey": null
  },
  "kind": "Request",
  "operation": {
    "argumentDefinitions": [],
    "kind": "Operation",
    "name": "BrandListQuery",
    "selections": (v0/*: any*/)
  },
  "params": {
    "cacheID": "60b819f8d33d67fe3b18b887784122f9",
    "id": null,
    "metadata": {},
    "name": "BrandListQuery",
    "operationKind": "query",
    "text": "query BrandListQuery {\n  brands(first: 1000) {\n    edges {\n      node {\n        id\n        name\n        urlSlug\n        privateLabel\n        status\n      }\n    }\n  }\n}\n"
  }
};
})();

(node as any).hash = "e329d955ac7c4c83c038f06eb56ee192";

export default node;
