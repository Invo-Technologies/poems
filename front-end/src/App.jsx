import Layout from './container/Layout';
import {
  AccountQuery,
  Generation,
  Aleo,
  Decrypt,
  PoemsReturn,
} from './components';
import { usePoemsContext } from './hooks/usePoemContext';

function App() {
  const { activeStep } = usePoemsContext();

  console.log(activeStep);
  return (
    <Layout>
      {activeStep === 'account' && <AccountQuery />}
      {activeStep === 'generation' && <Generation />}
      {activeStep === 'aleo' && <Aleo />}
      {activeStep === 'poemsreturn' && <PoemsReturn />}
      {activeStep === 'decrypt' && <Decrypt />}
    </Layout>
  );
}

export default App;
