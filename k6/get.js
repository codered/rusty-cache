import http from 'k6/http';
import { sleep } from 'k6';

export const options = {
    discardResponseBodies: true,
    scenarios: {
        contacts: {
            executor: 'constant-vus',
            vus: 10,
            duration: '30s',
        },
    },
};

export default function () {
    http.get('http://localhost:3030/v1/cache');
    sleep(1);
}