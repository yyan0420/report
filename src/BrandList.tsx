import React from 'react';
import { graphql, useLazyLoadQuery } from 'react-relay';

const BrandList: React.FC = () => {
  const data = useLazyLoadQuery(
    graphql`
      query BrandListQuery {
        brands(first: 1000) {
          edges {
            node {
              id
              name
              urlSlug
              privateLabel
              status
            }
          }
        }
      }
    `,
    {}
  );

  return (
    <div>
      <h1>Brand List</h1>
      <ul>
        {data.brands.edges?.map(({ node }) => (
          <li key={node.id}>
            <strong>ID:</strong> {node.id} <br />
            <strong>Name:</strong> {node.name} <br />
            <strong>URL Slug:</strong> {node.urlSlug} <br />
            <strong>Private Label:</strong> {node.privateLabel ? 'Yes' : 'No'} <br />
            <strong>Status:</strong> {node.status ? 'Active' : 'Inactive'}
          </li>
        ))}
      </ul>
    </div>
  );
};

export default BrandList;
