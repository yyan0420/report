/**
 * @generated SignedSource<<a70203ba22227666b1c843cafd6f85b6>>
 * @lightSyntaxTransform
 * @nogrep
 */

/* tslint:disable */
/* eslint-disable */
// @ts-nocheck

import type { ConcreteRequest } from 'relay-runtime';
export type TestQuery$variables = Record<PropertyKey, never>;
export type TestQuery$data = {
  readonly test: ReadonlyArray<{
    readonly name: string;
    readonly qty1: number;
  }>;
};
export type TestQuery = {
  response: TestQuery$data;
  variables: TestQuery$variables;
};

const node: ConcreteRequest = (function(){
var v0 = [
  {
    "alias": null,
    "args": null,
    "concreteType": "TestResult",
    "kind": "LinkedField",
    "name": "test",
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
        "name": "qty1",
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
    "name": "TestQuery",
    "selections": (v0/*: any*/),
    "type": "QueryRoot",
    "abstractKey": null
  },
  "kind": "Request",
  "operation": {
    "argumentDefinitions": [],
    "kind": "Operation",
    "name": "TestQuery",
    "selections": (v0/*: any*/)
  },
  "params": {
    "cacheID": "04321f0b7754be054e64e6d409a368c7",
    "id": null,
    "metadata": {},
    "name": "TestQuery",
    "operationKind": "query",
    "text": "query TestQuery {\n  test {\n    name\n    qty1\n  }\n}\n"
  }
};
})();

(node as any).hash = "aa50cead0081b4f841cb18b7bb68c23b";

export default node;
