import { useState } from 'react';
import {
  web3Accounts,
  web3Enable
} from '@polkadot/extension-dapp';
import clsx from 'clsx';
import { useNavigate } from 'react-router-dom';
import { useGlobalContext } from '../context';

const initialExtensionState = {
  data: undefined,
  loading: false,
  error: null
};

export const Connect = () => {
    console.log("hey")
    const navigate = useNavigate()
    const { contract, walletAddress, gameData, setShowAlert, setErrorMessage, setWalletAddress } = useGlobalContext();

  const [state, setState] = useState(initialExtensionState);

  const handleConnect = () => {
    setState({ ...initialExtensionState, loading: true });

    web3Enable('polkadot-extension-dapp-example')
      .then((injectedExtensions) => {
        if (!injectedExtensions.length) {
          return Promise.reject(new Error('NO_INJECTED_EXTENSIONS'));
        }

        return web3Accounts();
      })
      .then((accounts) => {
        console.log(accounts[ 0 ])
        if (!accounts.length) {
          return Promise.reject(new Error('NO_ACCOUNTS'));
        }

        setState({
          error: null,
          loading: false,
          data: {
            accounts: accounts,
            defaultAccount: accounts[ 0 ],
          }
        });
      })

      console.log("the state", state.data.defaultAccount.address)
      setWalletAddress(accounts[ 0 ].address)
      .catch((error) => {
        console.error('Error with connect', error);
        setState({ error, loading: false, data: undefined });
      });
  };

  if (state.error) {
    return (
      <span className="text-red-500 font-bold tracking-tight">
        Error with connect: {state.error.message}
      </span>
    );
  }

 
  if(state.data){
    navigate("/")
  }
  return state.data
    ? <>Hello, {beatifyAddress(state.data.defaultAccount.address)}!</>
    : <button
      disabled={state.loading}
      className={
        clsx(
          'inline-block rounded-lg px-4 py-1.5',
          'text-base font-semibold leading-7 text-white shadow-sm ring-1 ring-amber-600',
          state.loading ? 'cursor-not-allowed bg-amber-400' : 'bg-amber-500 hover:bg-amber-600 hover:ring-amber-600'
        )
      }
      onClick={handleConnect}
    >
      {state.loading ? 'Connecting...' : 'Connect'}
    </button>;
};

function beatifyAddress(address) {
  return `${address.slice(0, 3)}...${address.slice(-3)}`;
}