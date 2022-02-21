// SPDX-License-Identifier: GPL-3.0

pragma solidity 0.8.12;

 contract CIDStorage {
    string cid;

    constructor(string memory _cid) {
      cid = _cid;
   }

    function getCID() public view returns (string memory) {
        return cid;
    }
}
