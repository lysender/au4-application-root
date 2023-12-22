
import * as singleSpa from 'single-spa'
import * as isActive from './activity-fn.js'

function stripHash(str) {
  const chunks = str.split('#');
  if (chunks.length > 1) {
    return chunks[0];
  }
  return str;
}

singleSpa.addErrorHandler(err => {
  if (err.appOrParcelName === 'apm' && err.message.includes('died in status LOADING_SOURCE_CODE')) {
    if (console) {
      console.log(`Single-SPA Error: ${err.message}`);
      console.log(`Reloading page...`);
    }
    setTimeout(() => {
      window.location.href = stripHash(window.location.href.toString());
    }, 3000);
  } else {
    throw err;
  }
});

singleSpa.registerApplication('apm', () => window.System.import('@portal/apm'), isActive.apm);
singleSpa.start();
