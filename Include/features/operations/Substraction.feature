#Author: amandafloren11@gmail.com

@tag
Feature: Substraction

  Scenario Outline: Substraction
    Given The Calculator page is loaded successfully
    When <firstOperand> minus <secondOperand>
    Then I get the result <result>

    Examples: 
      | firstOperand  | secondOperand | result |
      | 3             | 2             | 1   |
      | 9             | 3             | 6   |
