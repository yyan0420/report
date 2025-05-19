import {
    Environment,
    Network,
    RecordSource,
    Store,
    type FetchFunction,
  } from 'relay-runtime';
  
  const fetchGraphQL: FetchFunction = async (operation, variables) => {
    const response = await fetch('http://localhost:3000/graphql', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        query: operation.text,
        variables,
      }),
    });
  
    return await response.json();
  };
  
  const environment = new Environment({
    network: Network.create(fetchGraphQL),
    store: new Store(new RecordSource()),
  });
  
  export default environment;
  