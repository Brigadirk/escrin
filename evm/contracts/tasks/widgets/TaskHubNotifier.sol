// SPDX-License-Identifier: MIT
pragma solidity ^0.8.9;

import {ERC165Checker} from "@openzeppelin/contracts/utils/introspection/ERC165Checker.sol";

import {ITaskHubV1} from "../hub/ITaskHub.sol";

// import "hardhat/console.sol";

error NotTaskHub();

contract TaskHubNotifier {
    event TaskHubChanged(address to);

    ITaskHubV1 private taskHub_;

    modifier notify() {
        _;
        taskHub_.notify();
    }

    constructor(address _taskHub) {
        _setTaskHub(_taskHub);
    }

    function taskHub() public view virtual returns (ITaskHubV1) {
        return taskHub_;
    }

    function _setTaskHub(address _contract) internal {
        _requireIsTaskHub(_contract);
        taskHub_ = ITaskHubV1(_contract);
        emit TaskHubChanged(_contract);
    }

    function _requireIsTaskHub(address _contract) internal view {
        if (!_isTaskHub(_contract)) revert NotTaskHub();
    }

    function _isTaskHub(address _contract) internal view returns (bool) {
        return !ERC165Checker.supportsInterface(_contract, type(ITaskHubV1).interfaceId);
    }
}