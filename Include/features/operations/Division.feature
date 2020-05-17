#Author: your.email@your.domain.com
#Keywords Summary :
#Feature: List of scenarios.
#Scenario: Business rule through list of steps with arguments.
#Given: Some precondition step
#When: Some key actions
#Then: To observe outcomes or validation
#And,But: To enumerate more Given,When,Then steps
#Scenario Outline: List of steps for data-driven as an Examples and <placeholder>
#Examples: Container for s table
#Background: List of steps run before each of the scenarios
#""" (Doc Strings)
#| (Data Tables)
#@ (Tags/Labels):To group Scenarios
#<> (placeholder)
#""
## (Comments)
#Sample Feature Definition Template
@tag
Feature: Division

  Scenario Outline: Division
    Given The Calculator page is loaded successfully
    When I devide <firstOperand> by <secondOperand>
    Then I get the result <result>

    Examples: 
      | firstOperand  | secondOperand | result |
      | 4             | 2             | 2   |
      | 9             | 3             | 3   |

