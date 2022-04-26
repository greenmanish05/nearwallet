import React from 'react';
import PropTypes from 'prop-types';
import Big from 'big.js';

export default function Form({ onSubmit, currentUser }) {
  return (
    <form onSubmit={onSubmit}>
      <fieldset id="fieldset">
        <p>Hello, { currentUser.accountId }!</p>
        <p>To get Near enter the key. To send Near enter the amount.</p>
        <p className="highlight">
          <label htmlFor="message">Key:</label>
          <input
            autoComplete="off"
            autoFocus
            id="key"
            //required
          />
        </p>
        <p>
          <label htmlFor="donation">Add amount:</label>
          <input
            autoComplete="off"
            defaultValue={'0'}
            id="amountnear"
            max={Big(currentUser.balance).div(10 ** 24)}
            min="0"
            step="0.1"
            type="number"
          />
          <span title="NEAR Tokens">Ⓝ</span>
        </p>
        <button type="submit">
          Get 1 NEAR
        </button>
        <button type="submit">
          Send NEAR
        </button>
      </fieldset>
    </form>
  );
}

Form.propTypes = {
  onSubmit: PropTypes.func.isRequired,
  currentUser: PropTypes.shape({
    accountId: PropTypes.string.isRequired,
    balance: PropTypes.string.isRequired
  })
};
