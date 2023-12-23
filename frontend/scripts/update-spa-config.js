
'use strict';

const fs = require('fs');
const path = require('path');

function copyStaticFile(source, target) {
  return new Promise((resolve, reject) => {
    fs.copyFile(source, target, (err) => {
      if (err) {
        reject(err);
      } else {
        resolve(true);
      }
    });
  });
}

function copyJsConfig(callback) {
  const prefix = (new Date()).getTime().toString();
  const source = path.resolve(__dirname, '../dist/root-config/single-spa.config.js');
  const target = path.resolve(__dirname, '../public/js/root-config/single-spa.config.js');
  const prefixedTarget = path.resolve(__dirname, '../public/js/root-config/' + prefix + '.single-spa.config.js');
  const url = '/js/root-config/' + prefix + '.single-spa.config.js';

  Promise.all([
    copyStaticFile(source, target),
    copyStaticFile(source, prefixedTarget),
  ]).then(() => {
    console.log('SPA config files copied!');
    callback.call(null, url);
  }).catch((err) => {
    console.log(err);
  });
}

function updateConfigJson(url) {
  const target = path.resolve(__dirname, '../spa-config.json');
  const data = { url };
  fs.writeFile(target, JSON.stringify(data), function(err) {
    if (err) {
      console.log(err);
    } else {
      console.log("Single SPA manifest updated!");
    }
  });
};

copyJsConfig(updateConfigJson);
