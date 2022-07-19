import React, { CSSProperties } from 'react';

export interface HomeProps {
  className?: string;
  style?: CSSProperties;
  children?: React.ReactNode;
}

export const Home: React.FC<HomeProps> = (props) => {
  const { className, style, children } = props;
  return (
    <div className={className} style={style}>
     {children}
    </div>
  );
};
