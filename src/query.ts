import { gql } from '@apollo/client';

export const GET_BRANDS = gql`
  query GetBrands {
    brand {
      id
      name
      urlSlug
      privateLabel
      status
    }
  }
`;
