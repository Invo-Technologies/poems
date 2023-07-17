import PropsTypes from 'prop-types';
import { Header } from '../components';

export default function Layout({ children }) {
  return (
    <div className="w-90% p-4 max-w-xl m-auto h-screen ">
      <Header />
      {children}
    </div>
  );
}

Layout.propTypes = {
  children: PropsTypes.node.isRequired,
};
