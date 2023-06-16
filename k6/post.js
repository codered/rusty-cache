import http from 'k6/http';
import { sleep } from 'k6';
import { randomSeed } from 'k6';
import { randomString } from 'https://jslib.k6.io/k6-utils/1.2.0/index.js';

export const options = {
    discardResponseBodies: true,
    scenarios: {
        contacts: {
            executor: 'constant-vus',
            vus: 10,
            duration: '5s',
        },
    },
};

export default function () {

    let random_key = randomString(10, `aeioubcdfghijpqrstuv`); //Math.random();
    let random_value = randomString(10, `aeioubcdfghijpqrstuv`); //Math.random();

    const url = 'http://localhost:3030/v1/cache';
    const payload = JSON.stringify({
        name: `${random_key}`,
        value: `${random_value}`,
    });

    const params = {
        headers: {
            'Content-Type': 'application/json',
        },
    };

    http.post(url, payload, params);
    // sleep(1);
}