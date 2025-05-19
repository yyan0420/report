/**
 * @generated SignedSource<<88a3b04a52f4a425b66f5c0324873e1b>>
 * @lightSyntaxTransform
 * @nogrep
 */

/* tslint:disable */
/* eslint-disable */
// @ts-nocheck

import type { ConcreteRequest } from 'relay-runtime';
export type FilterDrawerQuery$variables = Record<PropertyKey, never>;
export type FilterDrawerQuery$data = {
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
export type FilterDrawerQuery = {
  response: FilterDrawerQuery$data;
  variables: FilterDrawerQuery$variables;
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
    "name": "FilterDrawerQuery",
    "selections": (v0/*: any*/),
    "type": "QueryRoot",
    "abstractKey": null
  },
  "kind": "Request",
  "operation": {
    "argumentDefinitions": [],
    "kind": "Operation",
    "name": "FilterDrawerQuery",
    "selections": (v0/*: any*/)
  },
  "params": {
    "cacheID": "c9e79e15716fbd9a928d0cd343be3841",
    "id": null,
    "metadata": {},
    "name": "FilterDrawerQuery",
    "operationKind": "query",
    "text": "query FilterDrawerQuery {\n  brands(first: 1000) {\n    edges {\n      node {\n        id\n        name\n        urlSlug\n        privateLabel\n        status\n      }\n    }\n  }\n}\n"
  }
};
})();

(node as any).hash = "c652c7f684309353e33a8e0abccea528";

export default node;
