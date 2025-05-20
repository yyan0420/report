import React from "react";
import { graphql, useLazyLoadQuery } from 'react-relay';
import { type TestQuery } from "../__generated__/TestQuery.graphql";

const Test: React.FC = () => {
  const data = useLazyLoadQuery<TestQuery>(
    graphql`
      query TestQuery {
        test {
          name
          qty1
        }
      }
    `,
    {}
  );

  console.log(data)

  return (
    <div>
      Test
    </div>
  )
};

export default Test;
