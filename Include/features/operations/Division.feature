#Author: amandafloren11@gmail.com

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

