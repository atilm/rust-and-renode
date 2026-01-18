*** Settings ***
Suite Setup     Setup
Suite Teardown  Teardown
Test Teardown   Test Teardown
Resource        ${RENODEKEYWORDS}

*** Test Cases ***
Button Should Toggle LED
    ${x}=                      Execute Command         include @${CURDIR}/nucleo_robot_tests.resc
    Create Terminal Tester     sysbus.usart2    timeout=5    defaultPauseEmulation=true

    # Assert LED State         false    gpioa.greenled2
    Execute Command            gpioc.button Press
    # Assert LED State         true    gpioa.greenled2

    Wait For Line On Uart       LED TOGGLED

    # Alternatively, you can manually check the state and verify the value
    ${state}=    Execute Command    gpioa.greenled2 State
    Should Be Equal As Strings    ${state.strip()}    True