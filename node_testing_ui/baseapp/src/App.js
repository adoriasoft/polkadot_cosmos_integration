import React, { useState, createRef } from 'react';
import { Router, Route, Switch } from 'react-router-dom'
import { Dimmer, Loader, Grid, Sticky, Message } from 'semantic-ui-react';
import 'semantic-ui-css/semantic.min.css';

import { SubstrateContextProvider, DeveloperConsole, useSubstrate } from './substrate-lib';

import AccountSelector from './AccountSelector';
import ChainInfoPage from './pages/ChainInfoPage'
import SendExtrinsicPage from './pages/SendExtrinsicPage'

function Main () {
  const [accountAddress, setAccountAddress] = useState(null);
  const { apiState, keyring, keyringState, apiError } = useSubstrate();

  console.log(keyring, keyringState)

  const loader = text =>
    <Dimmer active>
      <Loader size='small'>{text}</Loader>
    </Dimmer>;

  const message = err =>
    <Grid centered columns={2} padded>
      <Grid.Column>
        <Message negative compact floating
          header='Connection error..'
          content={`${JSON.stringify(err, null, 4)}`}
        />
      </Grid.Column>
    </Grid>;

  if (apiState === 'ERROR') return message(apiError);
  else if (apiState !== 'READY') return loader('Connecting..');

  if (keyringState !== 'READY') {
    return loader('Loading accounts (please review any extension\'s authorization)');
  }

  const contextRef = createRef();

  return (
    <div ref={contextRef}>
      <Sticky context={contextRef}>
        <Router>
          <Switch>
            <Route path='/testing/chain_height' component={ChainInfoPage} />
            <Route path='/testing/send_ext' component={SendExtrinsicPage} />
          </Switch>
        </Router>
        <AccountSelector setAccountAddress={setAccountAddress} />
      </Sticky>
      <DeveloperConsole />
    </div>
  );
}

export default function App () {
  return (
    <SubstrateContextProvider>
      <Main />
    </SubstrateContextProvider>
  );
}
