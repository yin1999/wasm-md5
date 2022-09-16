/* tslint:disable */
/* eslint-disable */
/**
*/
export class Md5Digest {
  free(): void;
/**
*/
  constructor();
/**
* @param {Uint8Array} data
*/
  update(data: Uint8Array): void;
/**
* @returns {string}
*/
  finalize(): string;
/**
* @returns {Uint8Array}
*/
  finalize_bytes(): Uint8Array;
}
